@[extern "add_from_rust"]
opaque addFromRust : UInt32 → UInt32 → UInt32

-- @[extern "empty_array"]
-- opaque arrayFromRust : Array String

def main : IO Unit :=
  IO.println $ addFromRust 1 2
  -- IO.println $ arrayFromRust
