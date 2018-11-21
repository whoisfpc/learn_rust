#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro2::TokenStream;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    generate_impl(&ast).into()
}

fn generate_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);

    quote!{
        impl #ident #generics #where_clause {
            #[allow(unused_variables)]
            pub fn vertex_attrib_pointers(gl: &::gl::Gl) {
                let stride = ::std::mem::size_of::<Self>(); // byte offset between consecutive attributes
                let offset = 0;
                #(#fields_vertex_attrib_pointer)*
                // continue here
            }
        }
    }

}


fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<TokenStream> {
    match data {
        &syn::Data::Enum(_)
            => panic!("VertexAttribPointers can not be implemented for enums"),
        &syn::Data::Union(_)
            =>  panic!("VertexAttribPointers can not be implemented for Unit structs"),
        &syn::Data::Struct(syn::DataStruct{struct_token: _, ref fields, semi_token: _}) => {
            fields.iter()
                .map(generate_struct_field_vertex_attrib_pointer_call)
                .collect()
        },
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> TokenStream {
    //panic!("field = {:#?}", field)
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let location_attr = field.attrs
        .iter()
        .filter(|a| a.path.segments[0].ident == "location")
        .next()
        .unwrap_or_else(|| panic!(
            "Field {:?} is missing #[location = ?] attribute", field_name
        ));
    let location_value: usize = match location_attr.parse_meta() {
        Ok(syn::Meta::NameValue(syn::MetaNameValue{lit: syn::Lit::Str(lit_str), .. })) => lit_str.value().parse()
            .unwrap_or_else(
                |_| panic!("Field {} location attribute value must contain an integer", field_name)
            ),
        _ => panic!("Field {} location attribute value must be an integer literal", field_name)
    };

    let field_ty = &field.ty;
    quote! {
        let location = #location_value;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    }
}

