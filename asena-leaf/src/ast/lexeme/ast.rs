use std::any::Any;

use super::{maybe::Maybe, *};

impl<T: Terminal + 'static> Leaf for Lexeme<T> {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let spanned = token.clone();
        let terminal = <T as Terminal>::terminal(token)?;

        Some(Self {
            token: spanned,
            value: Maybe::Just(terminal),
        })
    }
}

impl<T: Leaf + 'static> Node for Lexeme<T> {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        match tree.into() {
            GreenTree::Leaf(ref leaf) => Self {
                token: get_single_token(leaf),
                value: Maybe::Just(T::make(GreenTree::Leaf(leaf.clone())).unwrap_or_default()),
            },
            GreenTree::None => Self {
                token: Default::default(),
                value: Maybe::Default(T::default()),
            },
            GreenTree::Token(lexeme) => {
                let value = match lexeme.value.downcast_ref::<T>() {
                    Some(value) => value.clone(),
                    None => return Default::default(),
                };

                Self {
                    token: lexeme.token,
                    value: Maybe::Just(value),
                }
            }
            _ => Self::default(),
        }
    }

    fn unwrap(self) -> GreenTree {
        GreenTree::Token(Lexeme {
            token: self.token,
            value: self.value.map(|value| Rc::new(value) as Rc<dyn Any>),
        })
    }
}

impl<T: Node> Node for Option<T> {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        let tree: GreenTree = tree.into();

        match tree {
            GreenTree::None => None,
            GreenTree::Empty => None,
            _ => Some(T::new(tree)),
        }
    }

    fn unwrap(self) -> GreenTree {
        match self {
            Some(vale) => vale.unwrap(),
            None => GreenTree::None,
        }
    }
}

impl<T> HasTokens for Lexeme<T> {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        self.token.tokens()
    }
}

impl<T> Located for Lexeme<T> {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Borrowed(&self.token.span)
    }
}

fn get_single_token(tree: &AstLeaf) -> Spanned<Token> {
    match tree.data.children.first() {
        Some(child) => match child.value {
            Child::Token(ref value) => tree.data.replace(value.clone()),
            Child::Tree(ref tree) => {
                #[cfg(debug_assertions)]
                println!("Lexeme::new: Leaf node has a tree child: {}", tree.kind);

                Default::default()
            }
        },
        None => {
            #[cfg(debug_assertions)]
            println!("Lexeme::new: Leaf node has no children: {}", tree.data.kind);

            Default::default()
        }
    }
}
