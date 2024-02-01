@[extern "add_from_rust"]
opaque addFromRust : UInt32 → UInt32 → UInt32

@[extern "my_empty_array"]
opaque arrayFromRust : UInt32 → UInt32 → Array String

def main : IO Unit := IO.println $ arrayFromRust 1 1
