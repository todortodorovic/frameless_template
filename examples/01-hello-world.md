# Example 1: Hello World

**Goal**: Add your first custom call to the runtime and see it work

## What you'll learn
- How to add new call variants to the runtime
- How to implement call logic
- How to rebuild and restart the node
- How to test your changes
- Basic runtime development workflow

## Prerequisites
- Completed the [Quick Start Guide](../../README.md#quick-start-guide)
- Node is running and accessible at `localhost:9944`

## Step 1: Understand the current state

Open `runtime/src/lib.rs` and find the `Call` enum (around line 185):

```rust
pub enum Call {
    Foo,
    SetValue(u32),
}
```

Currently we have:
- `Foo` - does nothing (no-op call)
- `SetValue(u32)` - stores a number in runtime state

## Step 2: Add your first call

Add a new variant to the `Call` enum:

```rust
pub enum Call {
    Foo,
    SetValue(u32),
    HelloWorld,  // ← Add this line!
}
```

## Step 3: Implement the logic

Find the `dispatch_extrinsic` function (around line 317) and add a new match arm:

```rust
fn dispatch_extrinsic(ext: BasicExtrinsic) -> DispatchResult {
    log::debug!(target: LOG_TARGET, "dispatching {:?}", ext);

    match ext.function {
        Call::Foo => {
            log::info!(target: LOG_TARGET, "Foo called");
        }
        Call::SetValue(v) => {
            log::info!(target: LOG_TARGET, "SetValue({}) called", v);
            sp_io::storage::set(VALUE_KEY, &v.encode());
        }
         // Add this block! start
        Call::HelloWorld => {
            log::info!(target: LOG_TARGET, "HelloWorld called");
            sp_io::storage::set(b"message", &"Hello World!".encode());
        }
        //end
    }

    Ok(())
}
```


## Step 4: Rebuild the runtime

**This step is crucial!** After changing runtime code, you must rebuild:

```bash
# Stop the node (Ctrl+C)
cargo build --release
# Restart the node
./target/release/solochain-template-node --dev
```

## Step 5: Test your changes

### First, verify the node is running:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"system_health",
  "params":[]
}' http://localhost:9944
```

### Submit your HelloWorld call:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0200"]
}' http://localhost:9944
```

**Expected response:**
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "result":"0xbb30a42c1e62f0afda5f0a4e8a562f7a13a24cea00ee81917b86b89e801314aa"
}
```

The `result` is the transaction hash - your call was successful!

### Check the stored message:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x6d657373616765"]
}' http://localhost:9944
```

**Expected response:**
```json
{
  "jsonrpc":"2.0",
  "id":1,
  "result":"0x3048656c6c6f20576f726c6421"
}
```

**`0x3048656c6c6f20576f726c6421`** contains:
- `0x30` = type identifier prefix (likely for String type)
- `0x48656c6c6f20576f726c6421` = SCALE-encoded `"Hello World!"`

**In your node, you’ll also be able to see that our transaction has been executed.**
```bash
2025-08-11 22:07:09 Entering initialize_block. header: <wasm:stripped> / version: 1    
2025-08-11 22:07:09 HelloWorld called    
2025-08-11 22:07:09 🎁 Prepared block for proposing at 2 (0 ms) [hash: 0x8fdcfeecde3df01b0f1ea10ce5843c00d91f2740415b23a848d0732202894970; parent_hash: 0xab51…aeaf; extrinsics_count: 1  
```
## Understanding the data

### Call encoding `0x0200`:
- `02` = Call index 2 (HelloWorld is the 3rd variant, 0-indexed)
- `00` = No parameters

### Storage key `0x6d657373616765`:
- This is the hex encoding of `"message"`

### Stored value `0x48656c6c6f20576f726c6421`:
- This is the SCALE-encoded string `"Hello World!"`

## What just happened?

1. ✅ You added a new call variant to the runtime
2. ✅ You implemented the logic to store a message
3. ✅ You rebuilt the runtime with your changes
4. ✅ You submitted the call via RPC
5. ✅ You verified the state change

## Troubleshooting

### "Call index out of bounds" error?
- **Did you rebuild?** Run `cargo build --release`
- **Did you restart the node?** Stop and restart after rebuild

### "Storage not found" error?
- Verify the call was executed successfully (check transaction hash)
- Check the storage key is correct

### "Client error: Could not decode Call, variant doesn't exist"?
- **Most common issue**: Node not rebuilt after code changes
- **Solution**: Always rebuild and restart after runtime changes

## Next steps

- Try adding a parameter to your call (like `HelloWorld(String)`)
- Add more complex logic (like storing multiple values)
- Check out [Example 2: Custom Storage](../02-custom-storage/) for more advanced features

## Need help?

- Open an issue with your error message
- Try the examples first