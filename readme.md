Run with 
```
RUST_LOG=debug cargo run
```

Test with
```
cat sample_orb_event.json | jq -c . > /tmp/sample_orb_event.json ; curl -X POST -H "Content-Type: application/json", -d @/tmp/sample_orb_event.json http://localhost:3000/my_event ; rm /tmp/sample_orb_event.json
```