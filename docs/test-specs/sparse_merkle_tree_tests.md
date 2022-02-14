# Sparse Merkle Tree Test specifications

## Root Signature Tests

1. [Test Empty Root](#test-empty-root)
2. [Test Root Update 1](#test-root-update-1)
3. [Test Root Update 2](#test-root-update-2)
4. [Test Root Update 3](#test-root-update-3)
5. [Test Root Update 5](#test-root-update-5)
---

### Test Empty Root

**Description**:

Tests the default root given no update or delete operations.

**Inputs**:

_No inputs_

**Outputs**:

- The expected root signature: `0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

**Example pseudocode**:
```py
smt = SparseMerkleTree.new(Storage.new(), sha256.new())
root = smt.root()
expected_root = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
expect(hex_encode(root), expected_root).to_be_equal
```
---

### Test Root Update 1

**Description**:

Tests the root after performing a single update call with the specified input.

**Inputs**:

1. Update the empty tree with leaf key `0u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)

**Outputs**:

- The expected root signature: `0x39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b`

**Example Pseudocode**:
```py
smt = SparseMerkleTree.new(Storage.new(), sha256.new())
smt.update(b"\x00\x00\x00\x00", b"DATA")
root = smt.root()
expected_root = '39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b'
expect(hex_encode(root), expected_root).to_be_equal
```
---

### Test Root Update 2

**Description**:

Tests the root after performing two update calls with the specified inputs.

**Inputs**:

1. Update the empty tree with leaf key `0u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
2. Update the tree with leaf key `1u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)

**Outputs**:

- The expected root signature: `0x8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb`

**Example Pseudocode**:
```py
smt = SparseMerkleTree.new(Storage.new(), sha256.new())
smt.update(b"\x00\x00\x00\x00", b"DATA")
smt.update(b"\x00\x00\x00\x01", b"DATA")
root = smt.root()
expected_root = '8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb'
expect(hex_encode(root), expected_root).to_be_equal
```
---

### Test Root Update 3

**Description**:

Tests the root after performing three update calls with the specified inputs.

**Inputs**:

1. Update the empty tree with leaf key `0u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
2. Update the tree with leaf key `1u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
3. Update the tree with leaf key `2u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)

**Outputs**:

- The expected root signature: `0x52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7`

**Example Pseudocode**:
```py
smt = SparseMerkleTree.new(Storage.new(), sha256.new())
smt.update(b"\x00\x00\x00\x00", b"DATA")
smt.update(b"\x00\x00\x00\x01", b"DATA")
smt.update(b"\x00\x00\x00\x02", b"DATA")
root = smt.root()
expected_root = '52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7'
expect(hex_encode(root), expected_root).to_be_equal
```
---

### Test Root Update 5

**Description**:

Tests the root after performing five update calls with the specified inputs.

**Inputs**:

1. Update the empty tree with leaf key `0u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
2. Update the tree with leaf key `1u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
3. Update the tree with leaf key `2u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
4. Update the tree with leaf key `3u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)
5. Update the tree with leaf key `4u32` (4 bytes, big endian) and leaf data `"DATA"` (bytes, UTF-8)

**Outputs**:

- The expected root signature: `0x108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b`

**Example Pseudocode**:
```py
smt = SparseMerkleTree.new(Storage.new(), sha256.new())

for i in 0..5:
    smt.update(&(i as u32).to_big_endian_bytes(), b"DATA")

root = smt.root()
expected_root = '108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b'
expect(hex_encode(root), expected_root).to_be_equal
```