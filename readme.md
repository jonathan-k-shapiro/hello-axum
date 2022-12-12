Run with 
```
RUST_LOG=debug cargo run
```

Test with
```
cat sample_orb_event.json | jq -c . > /tmp/sample_orb_event.json ; curl -X POST -H "Content-Type: application/json", -d @/tmp/sample_orb_event.json http://localhost:3000/my_event ; rm /tmp/sample_orb_event.json
```

## Using `ngrok` for testing

Create a personal GitHub repo to use as a source of webhooks. 

[Install ngrok](https://gist.github.com/wosephjeber/aa174fb851dfe87e644e)
```
brew install --cask ngrok
```

Run `ngrok` to route to port 3000
```
ngrok http 3000
```

Observe ngrok output to get webhook URL
```
Session Status         online                                                                                                                                
Account                jonathan.k.shapiro@gmail.com (Plan: Free)                                                                                             
Version                3.1.0                                                                                                                                 
Region                 United States (us)                                                                                                                    
Latency                36ms                                                                                                                                  
Web Interface          http://127.0.0.1:4040                                                                                                                 
Forwarding             https://1111-22-33-44-555.ngrok.io -> http://localhost:3000  
```

In your test GitHub repo, add a webhook.
* Set the webhook url to the value to left of `->` in the `Forwarding` line of ngrok output shown above
* Add a secret to your webhook to ensure you get a `x-hub-signature-256` header with requests. Genearate a secret any way you wish. I executed this in the terminal: `ruby -rsecurerandom -e 'puts SecureRandom.hex(20)'` MAKE SURE TO RECORD THE GENERATED SECRET IN BOTH GITHUB AND IN A `.env` file in this repo.

Run server code
```
RUST_LOG=debug cargo run
```


## VSCode rust extensions

Installed extensions
* Rust analyzer
* CodeLLDB (For debugging compiled languages generally. Works with rust.)

Added launch configuration in `.vscode/launch.json`
```
{
    "version": "0.2.0",
    "configurations": [

        {
            "name": "(OSX) Launch",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/hello-axum",
            "args": [],
            "cwd": "${workspaceRoot}",
        }
    ]
}
```