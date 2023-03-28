/* -------------------------------------------------------------------------- */
/*                          Recurrence Example Macro                          */
/* -------------------------------------------------------------------------- */

// a recurrence relation is a sequence where each value is defined in terms of one or more previous values
// with one or more initial values to get the whole thing started. 
// 
// For example: 
// The Fibonacci sequence can be defined by the relation:
// F​n​​=0,1,…,F​n−1​​+F​n-2​​
// Thus, the first two numbers in the sequence are 0 and 1, 
// with the third being F0 + F1 = 0 + 1 = 1, the fourth F1 + F2 = 1 + 1 = 2, and so on forever.

// And it includes the separators available for various fragment specifiers:

// expr and stmt may only be followed by one of: =>, ,, or ;.
// pat_param may only be followed by one of: =>, ,, =, |, if, or in.
// pat may only be followed by one of: =>, ,, =, if, or in.
// path and ty may only be followed by one of: =>, ,, =, |, ;, :, >, >>, [, {, as, where, or a macro variable of block fragment specifier.
// vis may only be followed by one of: ,, an identifier other than a non-raw priv, any token that can begin a type, or a metavariable with a ident, ty, or path fragment specifier.
// All other fragment specifiers have no restrictions.

macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence{
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ => $recur:expr  ) => 
    { 
        {

            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            // let n = self.pos; 
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            // a[n-1] + a[n-2]
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };    
}

fn main(){

    // let fib = recurrence![a[n]: f32 = 0.0, 1.0 ];
    let fib = recurrence![a[n]: u64 = 0, 1 => a[n-1] + a[n-2]];

    for e in fib.take(20) { println!("{}", e) }

}









mod macro_parsing_and_expansion{
    //  A macro invocation in Rust is, in contrast to something like C, 
    //  not a wholly separate pass over the source code. 
    //  Macro invocations are actually a normal part of the compiler's AST representation. 
    //  This means that invocations can only appear in positions where they're explicitly supported. 
    //  Currently, they can appear in place of items, methods, statements, expressions, and patterns. 
    //  Note that, as a consequence of this, there are some things you can't do with macros, 
    //  such as have one which expands to the identifier for a function declaration.
    //  
    //  However, the status of macro invocations as first-class members of the AST means 
    //  that the Rust parser has to be able to parse them into something sensible, 
    //  even when they use syntax that Rust itself doesn't support. 
    //  The way this is done is by parsing the contents of an invocation into "token trees".
    // 
    //  When it comes time to expand a macro invocation, 
    //  the compiler feeds the parsed token trees into the macro, 
    //  which must expand to a new sequence of token trees 
    //  which can be parsed as an AST node that matches the invocation's position.
    // 
    //  This means that not only is where you can use a macro restricted, 
    //  you also cannot have a macro which expands to something that isn't a complete, valid Rust construct.
    //  
    //  recurrence![ a[n]: $sty = 0, 1 ... a[n-1] + a[n-2] ]
    // the invocation arguments stored in the AST look something like:

    // [ `a` `[ ]` `:` `u64` `=` `0` `,` `1` `...` `a` `[ ]` `+` `a` `[ ]` ]
    //         ^                                         ^             |
    //      [ `n` ]                               [ `n` `-` `1` ]      ^
    //                                                          [ `n` `-` `2` ]
    //   
}

mod construction{
    // Usually, when working on a new macro, the first thing I do is decide what the macro invocation should look like.
    //
    // * the macro system will try to incrementally match the tokens provided as input to the macro against the provided rules.

}




