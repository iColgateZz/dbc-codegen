// Determining raw_type
// if SIG_VALTYPE_ exists:
//      if type == 0: 
//          goto integer
//      if type == 1:
//          raw_type = f32
//      if type == 2:
//          raw_type = f64
// else:
//      if signed:
//          raw_type = signed_int(size)
//      else:
//          raw_type = unsigned_int(size)

// Determining logical type
// if VAL_ exists:
//      logical_type = enum
// else:
//      just some value

// Determining physical type
// if logical_type == enum:
//      map integers to enum values
// else:
//      physical_type = float if (raw_type == (f32 or f64)) or factor is float or offset is float
//                      else integer, sign depends on raw_type, factor & offset

// With enums there should be 2 types:
// 1. Where the raw type allows for N values and enum has N entries
// 2. Where the raw type allows for N values and enum has <  N entries