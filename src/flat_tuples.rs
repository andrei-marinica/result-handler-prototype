pub trait TupleFlatPush<X> {
    type Output;

    fn flat_push(self, x: X) -> Self::Output;
}

impl<T> TupleFlatPush<T> for () {
    type Output = (T,);

    fn flat_push(self, t: T) -> Self::Output {
        (t,)
    }
}

impl<T, A> TupleFlatPush<T> for (A,) {
    type Output = (A, T);

    fn flat_push(self, t: T) -> Self::Output {
        (self.0, t)
    }
}

impl<T, A, B> TupleFlatPush<T> for (A, B) {
    type Output = (A, B, T);

    fn flat_push(self, t: T) -> Self::Output {
        (self.0, self.1, t)
    }
}

impl<T, A, B, C> TupleFlatPush<T> for (A, B, C) {
    type Output = (A, B, C, T);

    fn flat_push(self, t: T) -> Self::Output {
        (self.0, self.1, self.2, t)
    }
}


pub trait TupleUnpack {
    type Unpacked;

    fn tuple_unpack(self) -> Self::Unpacked;
}

impl TupleUnpack for () {
    type Unpacked = ();

    fn tuple_unpack(self) -> Self::Unpacked {}
}

impl<T> TupleUnpack for (T,) {
    type Unpacked = T;

    fn tuple_unpack(self) -> Self::Unpacked {
        self.0
    }
}

macro_rules! tuple_unpack_self {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TupleUnpack for ($($name,)+)
            {
				type Unpacked = Self;

                fn tuple_unpack(self) -> Self::Unpacked {
                    self
                }
            }
        )+
    }
}

tuple_unpack_self! {
    (0 T0 1 T1)
    (0 T0 1 T1 2 T2)
    (0 T0 1 T1 2 T2 3 T3)
    (0 T0 1 T1 2 T2 3 T3 4 T4)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

