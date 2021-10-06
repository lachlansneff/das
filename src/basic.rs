use std::{any::{Any, TypeId}, cmp::Ordering, fmt::Debug, ops::{Add, ControlFlow, Mul}};

use crate::{expr::{Expr, ExprRef}, functions, visitor::Visitor};

pub trait Basic: Send + Sync + Debug + Any {
    fn visit(self: ExprRef<Self>, visitor: &mut dyn Visitor) -> ControlFlow<()>;
    fn eq(&self, other: &dyn Basic) -> bool;
    /// In implementations of this, return `None` when
    /// the types do not match.
    fn cmp(&self, other: &dyn Basic) -> Option<Ordering>;
}

impl dyn Basic {
    pub fn is<T: Basic>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    pub fn downcast<T: Basic>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const dyn Basic as *const T))
            }
        } else {
            None
        }
    }
}

impl Expr {
    pub fn downcast_expr<'a, T: Basic>(&'a self) -> Result<Expr<T>, &'a Expr> {
        if self.is::<T>() {
            unsafe {
                Ok(Expr::clone(&*(self as *const Expr as *const Expr<T>)))
            }
        } else {
            Err(self)
        }
    }
}

impl<'a> ExprRef<'a> {
    pub fn downcast_exprref<T: Basic>(self) -> Result<ExprRef<'a, T>, ExprRef<'a>> {
        if self.is::<T>() {
            unsafe {
                Ok(*(&self as *const ExprRef as *const ExprRef<T>))
            }
        } else {
            Err(self)
        }
    }
}

impl<A, B> Add<B> for Expr<A>
where
    A: Basic,
    B: Into<Expr>,
{
    type Output = Expr;

    fn add(self, rhs: B) -> Self::Output {
        functions::plus((self as Expr).rf(), rhs.into().rf())
    }
}

impl<B> Add<B> for Expr
where
    B: Into<Expr>,
{
    type Output = Expr;

    fn add(self, rhs: B) -> Self::Output {
        functions::plus(self.rf(), rhs.into().rf())
    }
}

impl<A, B> Mul<B> for Expr<A>
where
    A: Basic,
    B: Into<Expr>
{
    type Output = Expr;

    fn mul(self, rhs: B) -> Self::Output {
        functions::times((self as Expr).rf(), rhs.into().rf())
    }
}

impl<B> Mul<B> for Expr
where
    B: Into<Expr>
{
    type Output = Expr;

    fn mul(self, rhs: B) -> Self::Output {
        functions::times(self.rf(), rhs.into().rf())
    }
}

impl PartialEq for dyn Basic {
    fn eq(&self, other: &Self) -> bool {
        <Self as Basic>::eq(self, other)
    }
}

impl Eq for dyn Basic {}

impl PartialOrd for dyn Basic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as Basic>::cmp(self, other)
    }
}

impl Ord for dyn Basic {
    fn cmp(&self, other: &Self) -> Ordering {
        <Self as Basic>::cmp(self, other).unwrap_or_else(|| {
            self.type_id().cmp(&other.type_id())
        })
    }
}
