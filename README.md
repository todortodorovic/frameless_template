# Frameless Template

A **learning-focused template** for building Substrate runtimes without FRAME. This template is designed to help you understand the fundamentals of blockchain runtime development through hands-on examples.

## 🎯 **What is Frameless Template?**

### **Traditional Substrate Development**
Most Substrate projects use **FRAME** - a framework that provides pre-built modules (pallets) for common blockchain functionality like accounts, balances, staking, etc. FRAME is powerful but can be overwhelming for beginners because it abstracts away many fundamental concepts.

### **What Makes This Template "Frameless"?**
This template **doesn't use FRAME at all**. Instead, it builds a Substrate runtime from scratch using only the core Substrate primitives. This means:

- **No pallets** - you write everything yourself
- **No macros** - you understand every line of code
- **No abstractions** - you see exactly how blockchain logic works
- **Full control** - you decide how to implement every feature

### **Why "Frameless" is Better for Learning**
1. **Fundamentals First**: You learn how Substrate actually works under the hood
2. **No Magic**: Every function, every storage operation, every call is visible
3. **Custom Everything**: You can implement features exactly how you want
4. **Deep Understanding**: You'll understand FRAME better after using this template
5. **Real Runtime**: This is a real, working blockchain runtime, not just examples

### **What You're Actually Building**
- **Runtime Logic**: The core business logic of your blockchain
- **Storage Management**: How data is stored and retrieved
- **Call Handling**: How external requests are processed
- **State Management**: How the blockchain state changes over time
- **Block Processing**: How blocks are created and finalized

### **Perfect For**
- **Beginners** who want to understand blockchain fundamentals
- **Developers** who want full control over their runtime
- **Students** learning about blockchain architecture
- **Anyone** who wants to see how Substrate works without FRAME

### **Not For**
- **Production applications** (use FRAME for that)
- **Quick prototypes** (FRAME is faster for simple things)
- **Complex DeFi protocols** (FRAME has battle-tested pallets)

## 🚀 **Quick Start Guide**

### Step 1: Get it running
```bash
git clone <repository-url>
cd frameless_template
cargo build --release
./target/release/solochain-template-node --dev
```

### Step 2: Test basic functionality
```bash
# Test the basic SetValue call
curl -H "Content-Type: application/json" -d '{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_submitExtrinsic",
  "params":["0x0139050000"]
}' http://localhost:9944
```

## 📚 **Learning Path - Examples**

We've created a series of examples that guide you from basic concepts to more advanced functionality:

### **Example 1: Hello World** ✅
- **Goal**: Add your first custom call to the runtime
- **Learn**: Basic runtime development, rebuild workflow
- **What you'll do**: Add a simple call that stores a message

### **Example 2: Simple Counter** ✅
- **Goal**: Build a counter with increment/decrement/reset
- **Learn**: Working with existing storage values, helper functions
- **What you'll do**: Create a counter that persists across blocks

### **Example 3: Custom Storage Types** ✅
- **Goal**: Store different data types (String, u32, bool)
- **Learn**: Multiple storage types, storage key management
- **What you'll do**: Store and retrieve different data types

### **Example 4: Simple Token System** ✅
- **Goal**: Build a basic token system with transfers and balance tracking
- **Learn**: Account management, balance tracking, transfer logic
- **What you'll do**: Create accounts, mint tokens, transfer between accounts

## 🔧 **How to Use This Template**

### **For Beginners**
1. Start with **Example 1: Hello World**
2. Follow each example step by step
3. Make small changes and see immediate results
4. Use the examples as a foundation for your own ideas

### **For Experienced Developers**
1. Use the examples as reference implementations
2. Modify and extend the functionality
3. Add your own custom calls and logic
4. Use this as a starting point for your own projects

## 🎓 **What You'll Learn**

- **Runtime Development**: How to build Substrate runtimes without FRAME
- **Storage Management**: How to store and retrieve data
- **Call Dispatching**: How to handle different types of calls
- **State Management**: How to manage runtime state
- **Testing**: How to test your changes and see results
- **Debugging**: How to troubleshoot and fix issues

## 🎯 **Why This Template?**

- **Learning Focused**: Designed for education, not production
- **Step by Step**: Progressive complexity from basic to advanced
- **Immediate Feedback**: See results of your changes right away
- **Well Documented**: Each example explains what and why
- **Extensible**: Easy to modify and extend

---

**Ready to start learning? Begin with [Example 1: Hello World](examples/01-hello-world/)!** 