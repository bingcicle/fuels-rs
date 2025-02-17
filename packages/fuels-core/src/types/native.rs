use crate::{Bits256, Identity, Parameterize, Token, Tokenizable};
use fuel_tx::{Address, AssetId, ContractId};
use fuels_types::{enum_variants::EnumVariants, errors::Error, param_types::ParamType};
use std::iter::zip;

impl<const SIZE: usize, T: Parameterize> Parameterize for [T; SIZE] {
    fn param_type() -> ParamType {
        ParamType::Array(Box::new(T::param_type()), SIZE)
    }
}

impl<T: Parameterize> Parameterize for Vec<T> {
    fn param_type() -> ParamType {
        ParamType::Vector(Box::new(T::param_type()))
    }
}

impl Parameterize for Address {
    fn param_type() -> ParamType {
        ParamType::Struct {
            name: "Address".to_string(),
            fields: vec![("0".to_string(), ParamType::B256)],
            generics: vec![],
        }
    }
}

impl Parameterize for ContractId {
    fn param_type() -> ParamType {
        ParamType::Struct {
            name: "ContractId".to_string(),
            fields: vec![("0".to_string(), ParamType::B256)],
            generics: vec![],
        }
    }
}

impl Parameterize for AssetId {
    fn param_type() -> ParamType {
        ParamType::Struct {
            name: "AssetId".to_string(),
            fields: vec![("0".to_string(), ParamType::B256)],
            generics: vec![],
        }
    }
}

impl Parameterize for () {
    fn param_type() -> ParamType {
        ParamType::Unit
    }
}

impl Parameterize for bool {
    fn param_type() -> ParamType {
        ParamType::Bool
    }
}

impl Parameterize for u8 {
    fn param_type() -> ParamType {
        ParamType::U8
    }
}

impl Parameterize for u16 {
    fn param_type() -> ParamType {
        ParamType::U16
    }
}

impl Parameterize for u32 {
    fn param_type() -> ParamType {
        ParamType::U32
    }
}

impl Parameterize for u64 {
    fn param_type() -> ParamType {
        ParamType::U64
    }
}

impl<T> Parameterize for Option<T>
where
    T: Parameterize + Tokenizable,
{
    fn param_type() -> ParamType {
        let param_types = vec![
            ("None".to_string(), ParamType::Unit),
            ("Some".to_string(), T::param_type()),
        ];
        let variants = EnumVariants::new(param_types)
            .expect("should never happen as we provided valid Option param types");
        ParamType::Enum {
            name: "Option".to_string(),
            variants,
            generics: vec![T::param_type()],
        }
    }
}

impl<T, E> Parameterize for Result<T, E>
where
    T: Parameterize + Tokenizable,
    E: Parameterize + Tokenizable,
{
    fn param_type() -> ParamType {
        let param_types = vec![T::param_type(), E::param_type()];
        let variant_param_types = zip(
            vec!["Ok".to_string(), "Err".to_string()],
            param_types.clone(),
        )
        .collect();
        let variants = EnumVariants::new(variant_param_types)
            .expect("should never happen as we provided valid Result param types");
        ParamType::Enum {
            name: "Result".to_string(),
            variants,
            generics: param_types,
        }
    }
}

impl Parameterize for Identity {
    fn param_type() -> ParamType {
        let variants = EnumVariants::new(vec![
            ("Address".to_string(), Address::param_type()),
            ("ContractId".to_string(), ContractId::param_type()),
        ])
        .expect("should never happen as we provided valid Identity param types");
        ParamType::Enum {
            name: "Identity".to_string(),
            variants,
            generics: vec![],
        }
    }
}

impl<T: Tokenizable> Tokenizable for Vec<T> {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Token::Vector(tokens) = token {
            tokens.into_iter().map(Tokenizable::from_token).collect()
        } else {
            Err(Error::InvalidData(format!(
                "Vec::from_token must only be given a Token::Vector. Got: {token}"
            )))
        }
    }

    fn into_token(self) -> Token {
        let tokens = self.into_iter().map(Tokenizable::into_token).collect();
        Token::Vector(tokens)
    }
}

impl Tokenizable for bool {
    fn from_token(token: Token) -> Result<Self, Error> {
        match token {
            Token::Bool(data) => Ok(data),
            other => Err(Error::InstantiationError(format!(
                "Expected `bool`, got {:?}",
                other
            ))),
        }
    }
    fn into_token(self) -> Token {
        Token::Bool(self)
    }
}

impl Tokenizable for () {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match token {
            Token::Unit => Ok(()),
            other => Err(Error::InstantiationError(format!(
                "Expected `Unit`, got {:?}",
                other
            ))),
        }
    }

    fn into_token(self) -> Token {
        Token::Unit
    }
}

impl Tokenizable for u8 {
    fn from_token(token: Token) -> Result<Self, Error> {
        match token {
            Token::U8(data) => Ok(data),
            other => Err(Error::InstantiationError(format!(
                "Expected `u8`, got {:?}",
                other
            ))),
        }
    }
    fn into_token(self) -> Token {
        Token::U8(self)
    }
}

impl Tokenizable for u16 {
    fn from_token(token: Token) -> Result<Self, Error> {
        match token {
            Token::U16(data) => Ok(data),
            other => Err(Error::InstantiationError(format!(
                "Expected `u16`, got {:?}",
                other
            ))),
        }
    }
    fn into_token(self) -> Token {
        Token::U16(self)
    }
}

impl Tokenizable for u32 {
    fn from_token(token: Token) -> Result<Self, Error> {
        match token {
            Token::U32(data) => Ok(data),
            other => Err(Error::InstantiationError(format!(
                "Expected `u32`, got {:?}",
                other
            ))),
        }
    }
    fn into_token(self) -> Token {
        Token::U32(self)
    }
}

impl Tokenizable for u64 {
    fn from_token(token: Token) -> Result<Self, Error> {
        match token {
            Token::U64(data) => Ok(data),
            other => Err(Error::InstantiationError(format!(
                "Expected `u64`, got {:?}",
                other
            ))),
        }
    }
    fn into_token(self) -> Token {
        Token::U64(self)
    }
}

// Here we implement `Tokenizable` for a given tuple of a given length.
// This is done this way because we can't use `impl<T> Tokenizable for (T,)`.
// So we implement `Tokenizable` for each tuple length, covering
// a reasonable range of tuple lengths.
macro_rules! impl_tuples {
    ($num: expr, $( $ty: ident : $no: tt, )+) => {
        impl<$($ty, )+> Tokenizable for ($($ty,)+) where
            $(
                $ty: Tokenizable,
            )+
        {
            fn from_token(token: Token) -> Result<Self, Error> {
                match token {
                    Token::Tuple(tokens) => {
                        let mut it = tokens.into_iter();
                        let mut next_token = move || {
                            it.next().ok_or_else(|| {
                                Error::InstantiationError("Ran out of tokens before tuple could be constructed".to_string())
                            })
                        };
                        Ok(($(
                          $ty::from_token(next_token()?)?,
                        )+))
                    },
                    other => Err(Error::InstantiationError(format!(
                        "Expected `Tuple`, got {:?}",
                        other,
                    ))),
                }
            }

            fn into_token(self) -> Token {
                Token::Tuple(vec![
                    $( self.$no.into_token(), )+
                ])
            }
        }

        impl<$($ty, )+> Parameterize for ($($ty,)+) where
            $(
                $ty: Parameterize,
            )+
        {
            fn param_type() -> ParamType {
                ParamType::Tuple(vec![
                    $( $ty::param_type(), )+
                ])
            }

        }
    }
}

// And where we actually implement the `Tokenizable` for tuples
// from size 1 to size 16.
impl_tuples!(1, A:0, );
impl_tuples!(2, A:0, B:1, );
impl_tuples!(3, A:0, B:1, C:2, );
impl_tuples!(4, A:0, B:1, C:2, D:3, );
impl_tuples!(5, A:0, B:1, C:2, D:3, E:4, );
impl_tuples!(6, A:0, B:1, C:2, D:3, E:4, F:5, );
impl_tuples!(7, A:0, B:1, C:2, D:3, E:4, F:5, G:6, );
impl_tuples!(8, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, );
impl_tuples!(9, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, );
impl_tuples!(10, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, );
impl_tuples!(11, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, );
impl_tuples!(12, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, );
impl_tuples!(13, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, );
impl_tuples!(14, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, );
impl_tuples!(15, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, );
impl_tuples!(16, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, );

impl Tokenizable for ContractId {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Token::Struct(tokens) = token {
            if let [Token::B256(data)] = tokens.as_slice() {
                Ok(ContractId::from(*data))
            } else {
                Err(Error::InstantiationError(format!(
                    "ContractId expected one `Token::B256`, got {tokens:?}"
                )))
            }
        } else {
            Err(Error::InstantiationError(format!(
                "Address expected `Token::Struct` got {token:?}",
            )))
        }
    }

    fn into_token(self) -> Token {
        let underlying_data: &[u8; 32] = &self;
        Token::Struct(vec![Bits256(*underlying_data).into_token()])
    }
}

impl Tokenizable for Address {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Token::Struct(tokens) = token {
            if let [Token::B256(data)] = tokens.as_slice() {
                Ok(Address::from(*data))
            } else {
                Err(Error::InstantiationError(format!(
                    "Address expected one `Token::B256`, got {tokens:?}"
                )))
            }
        } else {
            Err(Error::InstantiationError(format!(
                "Address expected `Token::Struct` got {token:?}",
            )))
        }
    }

    fn into_token(self) -> Token {
        let underlying_data: &[u8; 32] = &self;

        Token::Struct(vec![Bits256(*underlying_data).into_token()])
    }
}

impl Tokenizable for AssetId {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Token::Struct(tokens) = token {
            if let [Token::B256(data)] = tokens.as_slice() {
                Ok(AssetId::from(*data))
            } else {
                Err(Error::InstantiationError(format!(
                    "AssetId expected one `Token::B256`, got {tokens:?}"
                )))
            }
        } else {
            Err(Error::InstantiationError(format!(
                "AssetId expected `Token::Struct` got {token:?}",
            )))
        }
    }

    fn into_token(self) -> Token {
        let underlying_data: &[u8; 32] = &self;
        Token::Struct(vec![Bits256(*underlying_data).into_token()])
    }
}

impl<T> Tokenizable for Option<T>
where
    T: Parameterize + Tokenizable,
{
    fn from_token(token: Token) -> Result<Self, Error> {
        if let Token::Enum(enum_selector) = token {
            match *enum_selector {
                (0u8, _, _) => Ok(None),
                (1u8, token, _) => Ok(Option::<T>::Some(T::from_token(token)?)),
                (_, _, _) => Err(Error::InstantiationError(format!(
                    "Could not construct Option from enum_selector. Received: {:?}",
                    enum_selector
                ))),
            }
        } else {
            Err(Error::InstantiationError(format!(
                "Could not construct Option from token. Received: {token:?}",
            )))
        }
    }
    fn into_token(self) -> Token {
        let (dis, tok) = match self {
            None => (0u8, Token::Unit),
            Some(value) => (1u8, value.into_token()),
        };
        if let ParamType::Enum { variants, .. } = Self::param_type() {
            let selector = (dis, tok, variants);
            Token::Enum(Box::new(selector))
        } else {
            panic!("should never happen as Option::param_type() returns valid Enum variants");
        }
    }
}

impl<T, E> Tokenizable for Result<T, E>
where
    T: Parameterize + Tokenizable,
    E: Parameterize + Tokenizable,
{
    fn from_token(token: Token) -> Result<Self, Error> {
        if let Token::Enum(enum_selector) = token {
            match *enum_selector {
                (0u8, token, _) => Ok(Result::<T, E>::Ok(T::from_token(token)?)),
                (1u8, token, _) => Ok(Result::<T, E>::Err(E::from_token(token)?)),
                (_, _, _) => Err(Error::InstantiationError(format!(
                    "Could not construct Result from enum_selector. Received: {:?}",
                    enum_selector
                ))),
            }
        } else {
            Err(Error::InstantiationError(format!(
                "Could not construct Result from token. Received: {token:?}",
            )))
        }
    }
    fn into_token(self) -> Token {
        let (dis, tok) = match self {
            Ok(value) => (0u8, value.into_token()),
            Err(value) => (1u8, value.into_token()),
        };
        if let ParamType::Enum { variants, .. } = Self::param_type() {
            let selector = (dis, tok, variants);
            Token::Enum(Box::new(selector))
        } else {
            panic!("should never happen as Result::param_type() returns valid Enum variants");
        }
    }
}

impl Tokenizable for Identity {
    fn from_token(token: Token) -> Result<Self, Error> {
        if let Token::Enum(enum_selector) = token {
            match *enum_selector {
                (0u8, token, _) => Ok(Identity::Address(Address::from_token(token)?)),
                (1u8, token, _) => Ok(Identity::ContractId(ContractId::from_token(token)?)),
                (_, _, _) => Err(Error::InstantiationError(format!(
                    "Could not construct Identity from enum_selector. Received: {:?}",
                    enum_selector
                ))),
            }
        } else {
            Err(Error::InstantiationError(format!(
                "Could not construct Identity from token. Received: {token:?}",
            )))
        }
    }
    fn into_token(self) -> Token {
        let (dis, tok) = match self {
            Self::Address(value) => (0u8, value.into_token()),
            Self::ContractId(value) => (1u8, value.into_token()),
        };
        if let ParamType::Enum { variants, .. } = Self::param_type() {
            let selector = (dis, tok, variants);
            Token::Enum(Box::new(selector))
        } else {
            panic!("should never happen as Identity::param_type() returns valid Enum variants");
        }
    }
}

impl<const SIZE: usize, T: Tokenizable> Tokenizable for [T; SIZE] {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let gen_error = |reason| {
            Error::InvalidData(format!(
                "While constructing an array of size {SIZE}: {reason}"
            ))
        };

        match token {
            Token::Array(elements) => {
                let len = elements.len();
                if len != SIZE {
                    return Err(gen_error(format!(
                        "Was given a Token::Array with wrong number of elements: {len}"
                    )));
                }

                let detokenized = elements
                    .into_iter()
                    .map(Tokenizable::from_token)
                    .collect::<Result<Vec<T>, _>>()
                    .map_err(|err| {
                        gen_error(format!(", not all elements could be detokenized: {err}"))
                    })?;

                Ok(detokenized.try_into().unwrap_or_else(|_| {
                    panic!("This should never fail since we're checking the length beforehand.")
                }))
            }
            _ => Err(gen_error(format!("Expected a Token::Array, got {token}"))),
        }
    }

    fn into_token(self) -> Token {
        Token::Array(self.map(Tokenizable::into_token).to_vec())
    }
}
