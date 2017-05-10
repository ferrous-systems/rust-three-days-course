// [T; N] is a constructor, T → usize → 𝓤 
// (parameterize over T and you get A → 𝓤).
fn foo<const n: usize, const l: [u32; n]>() -> [u32; n] {
    // ^ note how l depends on n.
    l
}

// We know n from the length of the array.
let l = foo::<_, [1, 2, 3, 4, 5, 6]>();
//            ^   ^^^^^^^^^^^^^^^^