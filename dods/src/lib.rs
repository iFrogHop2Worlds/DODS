use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive a Struct-of-Arrays (SoA) container for a named-field struct.
///
/// This generates `FooSoA`, `FooRef`, `FooRefMut`, `FooSlice`, `FooSliceMut`,
/// `FooPtr`, and `FooPtrMut` for a `Foo` struct, plus a Vec-like API on `FooSoA`.
#[proc_macro_derive(SoA)]
pub fn dods_soa_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let soa_name = format_ident!("{}SoA", name);
    let soa_ref_name = format_ident!("{}Ref", name);
    let soa_ref_mut_name = format_ident!("{}RefMut", name);
    let soa_slice_name = format_ident!("{}Slice", name);
    let soa_slice_mut_name = format_ident!("{}SliceMut", name);
    let soa_ptr_name = format_ident!("{}Ptr", name);
    let soa_ptr_mut_name = format_ident!("{}PtrMut", name);

    let fields = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref f) => &f.named,
            _ => panic!("DODS SoA only supports structs with named fields"),
        },
        _ => panic!("DODS SoA only supports structs"),
    };

    let field_idents: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    // Use the first field to check lengths/emptiness for the whole SoA
    let first_field = field_idents[0];

    let iter_parts: Vec<_> = field_idents
        .iter()
        .map(|ident| quote! { self.#ident.iter() })
        .collect();
    let iter_mut_parts: Vec<_> = field_idents
        .iter()
        .map(|ident| quote! { self.#ident.iter_mut() })
        .collect();

    let mut iter_expr = iter_parts[0].clone();
    for part in iter_parts.iter().skip(1) {
        iter_expr = quote! { #iter_expr.zip(#part) };
    }

    let mut iter_mut_expr = iter_mut_parts[0].clone();
    for part in iter_mut_parts.iter().skip(1) {
        iter_mut_expr = quote! { #iter_mut_expr.zip(#part) };
    }

    let mut tuple_pattern = quote! { #first_field };
    for ident in field_idents.iter().skip(1) {
        tuple_pattern = quote! { (#tuple_pattern, #ident) };
    }

    let expanded = quote! {
        /// Struct-of-arrays container generated for the source struct.
        pub struct #soa_name {
            #( pub #field_idents: Vec<#field_types>, )*
        }

        /// Immutable references to a single element of the SoA.
        pub struct #soa_ref_name<'a> {
            #( pub #field_idents: &'a #field_types, )*
        }

        /// Mutable references to a single element of the SoA.
        pub struct #soa_ref_mut_name<'a> {
            #( pub #field_idents: &'a mut #field_types, )*
        }

        /// Immutable slices for each field over a range of the SoA.
        pub struct #soa_slice_name<'a> {
            #( pub #field_idents: &'a [#field_types], )*
        }

        /// Mutable slices for each field over a range of the SoA.
        pub struct #soa_slice_mut_name<'a> {
            #( pub #field_idents: &'a mut [#field_types], )*
        }

        /// Raw const pointers for each field buffer.
        pub struct #soa_ptr_name {
            #( pub #field_idents: *const #field_types, )*
        }

        /// Raw mut pointers for each field buffer.
        pub struct #soa_ptr_mut_name {
            #( pub #field_idents: *mut #field_types, )*
        }

        impl #soa_name {
            /// Creates a new, empty SoA.
            pub fn new() -> Self {
                Self {
                    #( #field_idents: Vec::new(), )*
                }
            }

            /// Creates an empty SoA with capacity for at least `capacity` elements.
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    #( #field_idents: Vec::with_capacity(capacity), )*
                }
            }

            /// Appends a single `#name` to the SoA.
            pub fn push(&mut self, item: #name) {
                #( self.#field_idents.push(item.#field_idents); )*
            }

            /// Inserts `element` at `index`, shifting later elements to the right.
            pub fn insert(&mut self, index: usize, element: #name) {
                let #name { #( #field_idents ),* } = element;
                #( self.#field_idents.insert(index, #field_idents); )*
            }

            /// Replaces the element at `index` with `element`, returning the old value.
            pub fn replace(&mut self, index: usize, element: #name) -> #name {
                let #name { #( #field_idents ),* } = element;
                #name {
                    #( #field_idents: std::mem::replace(&mut self.#field_idents[index], #field_idents), )*
                }
            }

            /// Removes and returns the element at `index`, shifting later elements left.
            pub fn remove(&mut self, index: usize) -> #name {
                #name {
                    #( #field_idents: self.#field_idents.remove(index), )*
                }
            }

            /// Removes the element at `index` in O(1) time.
            /// It swaps the element at `index` with the last element and then pops.
            /// Note: This does NOT preserve the order of elements.
            pub fn swap_remove(&mut self, index: usize) -> #name {
                #name {
                    #( #field_idents: self.#field_idents.swap_remove(index), )*
                }
            }

            pub fn pop(&mut self) -> Option<#name> {
                if self.#first_field.is_empty() { return None; }
                Some(#name {
                    #( #field_idents: self.#field_idents.pop().unwrap(), )*
                })
            }

            /// Returns the number of elements in the SoA.
            pub fn len(&self) -> usize {
                self.#first_field.len()
            }

            /// Returns true when the SoA has no elements.
            pub fn is_empty(&self) -> bool {
                self.#first_field.is_empty()
            }

            /// Returns the capacity of the underlying field vectors.
            pub fn capacity(&self) -> usize {
                self.#first_field.capacity()
            }

            /// Reserves capacity for at least `additional` more elements.
            pub fn reserve(&mut self, additional: usize) {
                #( self.#field_idents.reserve(additional); )*
            }

            /// Reserves the minimum capacity for `additional` more elements.
            pub fn reserve_exact(&mut self, additional: usize) {
                #( self.#field_idents.reserve_exact(additional); )*
            }

            /// Shrinks all field buffers as much as possible.
            pub fn shrink_to_fit(&mut self) {
                #( self.#field_idents.shrink_to_fit(); )*
            }

            /// Shortens the SoA to `len`, dropping excess elements.
            pub fn truncate(&mut self, len: usize) {
                #( self.#field_idents.truncate(len); )*
            }

            /// Clears all elements from the SoA.
            pub fn clear(&mut self) {
                #( self.#field_idents.clear(); )*
            }

            /// Appends all elements from `other`, leaving it empty.
            pub fn append(&mut self, other: &mut Self) {
                #( self.#field_idents.append(&mut other.#field_idents); )*
            }

            /// Splits the SoA into two at `at`, returning the tail.
            pub fn split_off(&mut self, at: usize) -> Self {
                Self {
                    #( #field_idents: self.#field_idents.split_off(at), )*
                }
            }

            /// Returns immutable slices of each field covering the full range.
            pub fn as_slice(&self) -> #soa_slice_name<'_> {
                #soa_slice_name {
                    #( #field_idents: &self.#field_idents[..], )*
                }
            }

            /// Returns mutable slices of each field covering the full range.
            pub fn as_mut_slice(&mut self) -> #soa_slice_mut_name<'_> {
                #soa_slice_mut_name {
                    #( #field_idents: &mut self.#field_idents[..], )*
                }
            }

            fn bounds_to_range(index: impl core::ops::RangeBounds<usize>, len: usize) -> (usize, usize) {
                use core::ops::Bound::{Excluded, Included, Unbounded};
                let start = match index.start_bound() {
                    Included(&n) => n,
                    Excluded(&n) => n + 1,
                    Unbounded => 0,
                };
                let end = match index.end_bound() {
                    Included(&n) => n + 1,
                    Excluded(&n) => n,
                    Unbounded => len,
                };
                (start, end)
            }

            /// Returns immutable slices of each field for the given `index` range.
            pub fn slice(&self, index: impl core::ops::RangeBounds<usize>) -> #soa_slice_name<'_> {
                let (start, end) = Self::bounds_to_range(index, self.len());
                #soa_slice_name {
                    #( #field_idents: &self.#field_idents[start..end], )*
                }
            }

            /// Returns mutable slices of each field for the given `index` range.
            pub fn slice_mut(&mut self, index: impl core::ops::RangeBounds<usize>) -> #soa_slice_mut_name<'_> {
                let (start, end) = Self::bounds_to_range(index, self.len());
                #soa_slice_mut_name {
                    #( #field_idents: &mut self.#field_idents[start..end], )*
                }
            }

            /// Returns raw const pointers to each field buffer.
            pub fn as_ptr(&self) -> #soa_ptr_name {
                #soa_ptr_name {
                    #( #field_idents: self.#field_idents.as_ptr(), )*
                }
            }

            /// Returns raw mut pointers to each field buffer.
            pub fn as_mut_ptr(&mut self) -> #soa_ptr_mut_name {
                #soa_ptr_mut_name {
                    #( #field_idents: self.#field_idents.as_mut_ptr(), )*
                }
            }

            /// Returns references to the element at `index`, or `None` if out of bounds.
            pub fn get(&self, index: usize) -> Option<#soa_ref_name<'_>> {
                if index >= self.#first_field.len() { return None; }
                Some(#soa_ref_name {
                    #( #field_idents: &self.#field_idents[index], )*
                })
            }

            /// Returns references to the element at `index`.
            ///
            /// # Panics
            /// Panics if `index` is out of bounds.
            pub fn index(&self, index: usize) -> #soa_ref_name<'_> {
                #soa_ref_name {
                    #( #field_idents: &self.#field_idents[index], )*
                }
            }

            /// Returns mutable references to the element at `index`, or `None` if out of bounds.
            pub fn get_mut(&mut self, index: usize) -> Option<#soa_ref_mut_name<'_>> {
                if index >= self.#first_field.len() { return None; }
                Some(#soa_ref_mut_name {
                    #( #field_idents: &mut self.#field_idents[index], )*
                })
            }

            /// Returns mutable references to the element at `index`.
            ///
            /// # Panics
            /// Panics if `index` is out of bounds.
            pub fn index_mut(&mut self, index: usize) -> #soa_ref_mut_name<'_> {
                #soa_ref_mut_name {
                    #( #field_idents: &mut self.#field_idents[index], )*
                }
            }

            /// Returns the first element, if any.
            pub fn first(&self) -> Option<#soa_ref_name<'_>> {
                self.get(0)
            }

            /// Returns the last element, if any.
            pub fn last(&self) -> Option<#soa_ref_name<'_>> {
                self.get(self.len().saturating_sub(1))
            }

            /// Returns mutable references to the first element, if any.
            pub fn first_mut(&mut self) -> Option<#soa_ref_mut_name<'_>> {
                self.get_mut(0)
            }

            /// Returns mutable references to the last element, if any.
            pub fn last_mut(&mut self) -> Option<#soa_ref_mut_name<'_>> {
                self.get_mut(self.len().saturating_sub(1))
            }

            /// Reorders the SoA using `indices` as the new-to-old mapping.
            ///
            /// Each output position `i` takes its element from `indices[i]`.
            pub fn apply_index(&mut self, indices: &[usize]) {
                let len = self.len();
                if indices.len() != len {
                    panic!("index length mismatch");
                }
                let mut seen = vec![false; len];
                for &idx in indices {
                    if idx >= len || seen[idx] {
                        panic!("indices must be a permutation");
                    }
                    seen[idx] = true;
                }
                let mut permutation = vec![0usize; len];
                for (new_pos, &old_pos) in indices.iter().enumerate() {
                    permutation[old_pos] = new_pos;
                }
                for i in 0..len {
                    while permutation[i] != i {
                        let j = permutation[i];
                        #( self.#field_idents.swap(i, j); )*
                        permutation.swap(i, j);
                    }
                }
            }

            pub fn sort_by<F>(&mut self, mut f: F)
            where
                F: FnMut(#soa_ref_name<'_>, #soa_ref_name<'_>) -> std::cmp::Ordering,
            {
                let mut permutation: Vec<usize> = (0..self.len()).collect();
                permutation.sort_by(|j, k| f(self.index(*j), self.index(*k)));

                self.apply_index(&permutation);
            }

            pub fn sort_by_key<F, K>(&mut self, mut f: F)
            where
                F: FnMut(#soa_ref_name<'_>) -> K,
                K: Ord,
            {
                let mut permutation: Vec<usize> = (0..self.len()).collect();
                permutation.sort_by_key(|j| f(self.index(*j)));

                self.apply_index(&permutation);
            }

            /// Returns an iterator over immutable references to each element.
            pub fn iter(&self) -> impl Iterator<Item = #soa_ref_name<'_>> {
                #iter_expr.map(|#tuple_pattern| #soa_ref_name {
                    #( #field_idents: #field_idents, )*
                })
            }

            /// Returns an iterator over mutable references to each element.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = #soa_ref_mut_name<'_>> {
                #iter_mut_expr.map(|#tuple_pattern| #soa_ref_mut_name {
                    #( #field_idents: #field_idents, )*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
