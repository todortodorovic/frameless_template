# Example 3: Custom Storage Types

**Goal**: Store different data types and learn storage key management

## What you'll learn
- How to store different data types (String, u32, bool)
- How to manage multiple storage keys
- How to use `mutate_state` with different types
- Storage key naming conventions
- Type-safe storage operations

## Prerequisites
- Completed [Example 2: Simple Counter](../02-simple-counter/)
- Node is running and accessible

## Step 1: Add storage type calls

Add these variants to the `Call` enum:

```rust
pub enum Call {
    Foo,
    SetValue(u32),
    HelloWorld,
    Increment,
    Decrement,
    Reset,
    StoreString(String),      // ← Add these
    StoreNumber(u32),         // ← three lines
    StoreBoolean(bool),       // ←
    GetStorageInfo,           // ←
}
```

## Step 2: Implement storage logic

Add these match arms to `dispatch_extrinsic`:

```rust
Call::StoreString(message) => {
    log::info!(target: LOG_TARGET, "StoreString({}) called", message);
    sp_io::storage::set(b"stored_string", &message.encode());
    log::info!(target: LOG_TARGET, "String stored successfully");
}
Call::StoreNumber(value) => {
    log::info!(target: LOG_TARGET, "StoreNumber({}) called", value);
    sp_io::storage::set(b"stored_number", &value.encode());
    log::info!(target: LOG_TARGET, "Number stored successfully");
}
Call::StoreBoolean(flag) => {
    log::info!(target: LOG_TARGET, "StoreBoolean({}) called", flag);
    sp_io::storage::set(b"stored_boolean", &flag.encode());
    log::info!(target: LOG_TARGET, "Boolean stored successfully");
}
Call::GetStorageInfo => {
    log::info!(target: LOG_TARGET, "GetStorageInfo called");
    
    // Read all stored values and log them
    let string_value = Self::get_state::<String>(b"stored_string");
    let number_value = Self::get_state::<u32>(b"stored_number");
    let boolean_value = Self::get_state::<bool>(b"stored_boolean");
    
    log::info!(target: LOG_TARGET, "Storage contents:");
    log::info!(target: LOG_TARGET, "  String: {:?}", string_value);
    log::info!(target: LOG_TARGET, "  Number: {:?}", number_value);
    log::info!(target: LOG_TARGET, "  Boolean: {:?}", boolean_value);
}
```

## Step 3: Initialize storage in genesis

Add these lines to `do_build_state`:

```rust
pub(crate) fn do_build_state(runtime_genesis: RuntimeGenesis) -> sp_genesis_builder::Result {
    sp_io::storage::set(&VALUE_KEY, &runtime_genesis.value.encode());
    sp_io::storage::set(b"counter", &0u32.encode());
    
    // Add these lines for custom storage types
    sp_io::storage::set(b"stored_string", &"Hello from Genesis!".encode());
    sp_io::storage::set(b"stored_number", &42u32.encode());
    sp_io::storage::set(b"stored_boolean", &true.encode());
    
    Ok(())
}
```

## Step 4: Rebuild and test

```bash
cargo build --release
./target/release/solochain-template-node --dev
```

## Step 5: Test your storage types

### Check initial genesis values:
```bash
# Check stored string
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x73746f7265645f737472696e67"]
}' http://localhost:9944

# Check stored number
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x73746f7265645f6e756d626572"]
}' http://localhost:9944

# Check stored boolean
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"state_getStorage",
  "params":["0x73746f7265645f626f6f6c65616e"]
}' http://localhost:9944
```

### Store a new string:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x061c4d79206e657720737472696e67"]
}' http://localhost:9944
```

**Explanation of `0x061c4d79206e657720737472696e67`:**
- `06` = Call index 6 (StoreString is the 7th variant, 0-indexed)
- `1c` = String length (28 bytes)
- `4d79206e657720737472696e67` = "My new string" in hex

### Store a new number:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0789000000"]
}' http://localhost:9944
```

**Explanation of `0x0789000000`:**
- `07` = Call index 7 (StoreNumber is the 8th variant, 0-indexed)
- `89000000` = `137` in little-endian format

### Store a new boolean:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0800"]
}' http://localhost:9944
```

**Explanation of `0x0800`:**
- `08` = Call index 8 (StoreBoolean is the 9th variant, 0-indexed)
- `00` = `false` (01 would be `true`)

### Get storage info:
```bash
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0900"]
}' http://localhost:9944
```

**Explanation of `0x0900`:**
- `09` = Call index 9 (GetStorageInfo is the 10th variant, 0-indexed)
- `00` = No parameters

**In your node, you’ll also be able to see that our transaction has been executed.**
```bash
2025-08-11 23:47:27 Entering initialize_block. header: <wasm:stripped> / version: 1    
2025-08-11 23:47:27 GetStorageInfo called    
2025-08-11 23:47:27 Storage contents:    
2025-08-11 23:47:27   String: Some("Hello from Genesis!")    
2025-08-11 23:47:27   Number: Some(137)    
2025-08-11 23:47:27   Boolean: Some(true)    
2025-08-11 23:47:27 🎁 Prepared block for proposing at 67 (0 ms) [hash: 0xa4aa51797157b045bd4850a21a63f9422b78152e4ec9fdbf21f5852bf655103b; parent_hash: 0x6695…6727; extrinsics_count: 1    
2025-08-11 23:47:27
```

## Understanding the data

### Call indices:
- `StoreString(String)` = index 6 → `0x06`
- `StoreNumber(u32)` = index 7 → `0x07`
- `StoreBoolean(bool)` = index 8 → `0x08`
- `GetStorageInfo` = index 9 → `0x09`

### Storage keys:
- `"stored_string"` → `0x73746f7265645f737472696e67`
- `"stored_number"` → `0x73746f7265645f6e756d626572`
- `"stored_boolean"` → `0x73746f7265645f626f6f6c65616e`

### Data encoding:
- **String**: `"Hello"` → `0x0548656c6c6f` (length + UTF-8 bytes)
- **Number**: `137` → `0x89000000` (little-endian u32)
- **Boolean**: `true` → `0x01`, `false` → `0x00`

## Key concepts learned

1. **Multiple storage keys**: Use different keys for different data types
2. **Type encoding**: Each type has its own encoding format
3. **Genesis initialization**: Set up initial values when the chain starts
4. **Storage key management**: Consistent naming conventions
5. **Type safety**: Runtime ensures correct types are stored/retrieved



## Next steps

- Add validation (e.g., string length limits)
- Implement storage cleanup functions
- Try [Example 4: Simple Token](../04-simple-token/) for account-based storage
- Add events for storage changes

## Troubleshooting

### "Call index out of bounds" error?
- **Did you rebuild?** Run `cargo build --release`
- **Did you restart the node?** Stop and restart after rebuild

### "Storage not found" error?
- Verify the call was executed successfully (check transaction hash)
- Check the storage key is correct
- Ensure genesis initialization ran

### "Could not decode" error?
- Check the data format matches the expected type
- Verify the encoding is correct

## Need help?

- Open an issue with your error message
- Try the examples first