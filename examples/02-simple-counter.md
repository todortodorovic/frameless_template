# Example 2: Simple Counter

**Goal**: Build a counter that can increment, decrement, and reset

## What you'll learn
- How to work with existing storage values
- How to use `mutate_state` for reading and updating
- How to implement multiple related calls
- How to handle different data types in storage

## Prerequisites
- Completed [Example 1: Hello World](../01-hello-world/)
- Node is running and accessible

## Step 1: Add counter calls

Add these variants to the `Call` enum:

```rust
pub enum Call {
    Foo,
    SetValue(u32),
    HelloWorld,
    Increment,      // ← Add these
    Decrement,      // ← three lines
    Reset,          // ←
}
```

## Step 2: Implement counter logic

Add these match arms to `dispatch_extrinsic`:

```rust
Call::Increment => {
    log::info!(target: LOG_TARGET, "Increment called");
    Self::mutate_state(b"counter", |counter: &mut u32| {
        *counter += 1;
        log::info!(target: LOG_TARGET, "Counter incremented to: {}", *counter);
    });
}
Call::Decrement => {
    log::info!(target: LOG_TARGET, "Decrement called");
    Self::mutate_state(b"counter", |counter: &mut u32| {
        if *counter > 0 {
            *counter -= 1;
        }
        log::info!(target: LOG_TARGET, "Counter decremented to: {}", *counter);
    });
}
Call::Reset => {
    log::info!(target: LOG_TARGET, "Reset called");
    Self::mutate_state(b"counter", |counter: &mut u32| {
        *counter = 0;
        log::info!(target: LOG_TARGET, "Counter reset to: {}", *counter);
    });
}
```

## Step 3: Initialize counter in genesis

Find the `do_build_state` function and add counter initialization:

```rust
pub(crate) fn do_build_state(runtime_genesis: RuntimeGenesis) -> sp_genesis_builder::Result {
    sp_io::storage::set(&VALUE_KEY, &runtime_genesis.value.encode());
    // Add this line to initialize counter
    sp_io::storage::set(b"counter", &0u32.encode());
    Ok(())
}
```

## Step 4: Rebuild and test

```bash
cargo build --release
./target/release/solochain-template-node --dev
```

## Step 5: Test your counter

### Check initial counter value:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x636f756e746572"]
}' http://localhost:9944
```

**Expected**: `0x00000000` (counter starts at 0)

### Increment the counter:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0300"]
}' http://localhost:9944
```

### Check counter value:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x636f756e746572"]
}' http://localhost:9944
```

**Expected**: `0x01000000` (counter is now 1)

### Decrement the counter:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0400"]
}' http://localhost:9944
```

### Reset the counter:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0500"]
}' http://localhost:9944
```

## Understanding the data

### Call indices:
- `HelloWold` = index 2 → `0x02`
- `Increment` = index 3 → `0x03`
- `Decrement` = index 4 → `0x04`
- `Reset` = index 5 → `0x05`

### Storage key:
- `"counter"` → `0x636f756e746572`

### Counter values:
- `0` → `0x00000000`
- `1` → `0x01000000`
- `2` → `0x02000000`

## Key concepts learned

1. **`mutate_state` usage**: Read, modify, and write back in one operation
2. **Genesis initialization**: Set up initial state when the chain starts
3. **Multiple related calls**: Build a cohesive feature with multiple operations
4. **Storage key management**: Use consistent keys for related data

## Next steps

- Add bounds checking (prevent negative numbers)
- Add events/logging for better debugging
- Try [Example 3: Simple Token](../03-simple-token/) for more complex logic
