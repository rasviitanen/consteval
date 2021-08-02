#[macro_export]
macro_rules! const_eval {
    ($($tree: tt)*) => {{
        consteval_proc::const_eval! {
            extern crate proc_macro;
            extern crate quote;
            use quote::ToTokens;
            use proc_macro::{TokenStream};

            #[proc_macro]
            #[allow(dead_code)]
            pub fn const_value(_ts: TokenStream) -> TokenStream {
                let tree = {$($tree)*};
                let res = quote::quote! {
                    #tree
                };
                res.to_token_stream().into()
            }
        }

        #[cfg(second_compilation)]
        const_crimes::const_value!()
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let v = const_eval!(
            let mut res = 0;
            for i in 0..200 {
                if i % 3 == 0 {
                    res += 1;
                }
            }
            res.to_string().parse::<i32>().unwrap()
        );
        assert_eq!(v, 67);
    }
}
