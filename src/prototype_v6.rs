use std::marker::PhantomData;

use crate::nested_tuples::{Flatten, NestedTupleAppend};

pub trait RetListItem<Original> {
    type Returns;

    fn single_return(&self) -> Self::Returns;
}

impl<Original> RetListItem<Original> for () {
    type Returns = ();

    fn single_return(&self) -> Self::Returns {}
}

pub struct PrintMessage(&'static str);

impl<Original> RetListItem<Original> for PrintMessage {
    type Returns = ();

    fn single_return(&self) -> Self::Returns {
        println!("{}", self.0)
    }
}

pub struct ReturnsStr(&'static str);

impl RetListItem<i32> for ReturnsStr {
    type Returns = &'static str;

    fn single_return(&self) -> Self::Returns {
        println!("returning str: {} ...", self.0);
        self.0
    }
}

pub struct ReturnsInt(i32);

impl RetListItem<i32> for ReturnsInt {
    type Returns = i32;

    fn single_return(&self) -> Self::Returns {
        println!("returning i32: {} ...", self.0);
        self.0
    }
}

pub struct ReturnsDefault;

impl<D: Default> RetListItem<D> for ReturnsDefault {
    type Returns = D;

    fn single_return(&self) -> Self::Returns {
        D::default()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub trait RetList {
    type Original;
    type ListReturn;

    fn list_return(&self) -> Self::ListReturn;
}

pub trait RetListAppendRet<T>: RetList
where
    T: RetListItem<Self::Original>,
{
    type RetOutput: RetList<Original = Self::Original>;

    fn append_ret(self, t: T) -> Self::RetOutput;
}

pub trait RetListAppendNoRet<T>: RetList
where
    T: RetListItem<Self::Original, Returns = ()>,
{
    type NoRetOutput: RetList<Original = Self::Original>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput;
}

impl RetList for () {
    type Original = ();
    type ListReturn = ();

    fn list_return(&self) -> Self::ListReturn {}
}

impl<T> RetListAppendRet<T> for ()
where
    T: RetListItem<()>,
{
    type RetOutput = ConsRet<T, ()>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet { head: t, tail: () }
    }
}

impl<T> RetListAppendNoRet<T> for ()
where
    T: RetListItem<(), Returns = ()>,
{
    type NoRetOutput = ConsNoRet<T, ()>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet { head: t, tail: () }
    }
}

pub struct OriginalMarker<O> {
    _phantom: PhantomData<O>,
}

impl<O> Default for OriginalMarker<O> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<O> RetList for OriginalMarker<O> {
    type Original = O;
    type ListReturn = ();

    fn list_return(&self) -> Self::ListReturn {}
}

impl<O, T> RetListAppendRet<T> for OriginalMarker<O>
where
    T: RetListItem<O>,
{
    type RetOutput = ConsRet<T, OriginalMarker<O>>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet {
            head: t,
            tail: self,
        }
    }
}

impl<O, T> RetListAppendNoRet<T> for OriginalMarker<O>
where
    T: RetListItem<O, Returns = ()>,
{
    type NoRetOutput = ConsNoRet<T, OriginalMarker<O>>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet {
            head: t,
            tail: self,
        }
    }
}

pub struct ConsRet<Head, Tail>
where
    Head: RetListItem<Tail::Original>,
    Tail: RetList,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsRet<Head, Tail>
where
    Head: RetListItem<Tail::Original>,
    Tail: RetList,
{
    type Original = Tail::Original;
    type ListReturn = (Head::Returns, Tail::ListReturn);

    fn list_return(&self) -> Self::ListReturn {
        let head_ret = self.head.single_return();
        let tail_ret = self.tail.list_return();
        (head_ret, tail_ret)
    }
}

impl<Head, Tail, T> RetListAppendRet<T> for ConsRet<Head, Tail>
where
    Head: RetListItem<Tail::Original>,
    Tail: RetList + RetListAppendRet<T>,
    T: RetListItem<Tail::Original>,
{
    type RetOutput = ConsRet<Head, <Tail as RetListAppendRet<T>>::RetOutput>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsRet {
            head: self.head,
            tail: self.tail.append_ret(t),
        }
    }
}

impl<Head, Tail, T> RetListAppendNoRet<T> for ConsRet<Head, Tail>
where
    Head: RetListItem<Tail::Original>,
    Tail: RetList + RetListAppendNoRet<T>,
    T: RetListItem<Tail::Original, Returns = ()>,
{
    type NoRetOutput = ConsRet<Head, <Tail as RetListAppendNoRet<T>>::NoRetOutput>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsRet {
            head: self.head,
            tail: self.tail.append_no_ret(t),
        }
    }
}

/// Handlers that return nothing.
pub struct ConsNoRet<Head, Tail>
where
    Head: RetListItem<Tail::Original, Returns = ()>,
    Tail: RetList,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsNoRet<Head, Tail>
where
    Head: RetListItem<Tail::Original, Returns = ()>,
    Tail: RetList,
{
    type Original = Tail::Original;
    type ListReturn = Tail::ListReturn;

    fn list_return(&self) -> Self::ListReturn {
        self.head.single_return();
        self.tail.list_return()
    }
}

impl<Head, Tail, T> RetListAppendRet<T> for ConsNoRet<Head, Tail>
where
    Head: RetListItem<Tail::Original, Returns = ()>,
    Tail: RetList + RetListAppendRet<T>,
    T: RetListItem<Tail::Original>,
{
    type RetOutput = ConsNoRet<Head, <Tail as RetListAppendRet<T>>::RetOutput>;

    fn append_ret(self, t: T) -> Self::RetOutput {
        ConsNoRet {
            head: self.head,
            tail: self.tail.append_ret(t),
        }
    }
}

impl<Head, Tail, T> RetListAppendNoRet<T> for ConsNoRet<Head, Tail>
where
    Head: RetListItem<Tail::Original, Returns = ()>,
    Tail: RetList + RetListAppendNoRet<T>,
    T: RetListItem<Tail::Original, Returns = ()>,
{
    type NoRetOutput = ConsNoRet<Head, <Tail as RetListAppendNoRet<T>>::NoRetOutput>;

    fn append_no_ret(self, t: T) -> Self::NoRetOutput {
        ConsNoRet {
            head: self.head,
            tail: self.tail.append_no_ret(t),
        }
    }
}

pub struct ListWrapper<L: RetList>(L);

pub fn new_list() -> ListWrapper<()> {
    ListWrapper(())
}

impl ListWrapper<()> {
    pub fn original_marker<O>(self) -> ListWrapper<OriginalMarker<O>> {
        ListWrapper(OriginalMarker::default())
    }
}

impl<L: RetList> ListWrapper<L> {
    pub fn eval(&self) -> <L::ListReturn as Flatten>::Unpacked
    where
        L::ListReturn: Flatten,
    {
        self.0.list_return().flatten_unpack()
    }

    pub fn returns<T>(self, rh: T) -> ListWrapper<L::RetOutput>
    where
        T: RetListItem<L::Original>,
        L: RetListAppendRet<T>,
    {
        ListWrapper(self.0.append_ret(rh))
    }

    pub fn handle_result<T>(self, t: T) -> ListWrapper<L::NoRetOutput>
    where
        T: RetListItem<L::Original, Returns = ()>,
        L: RetListAppendNoRet<T>,
    {
        ListWrapper(self.0.append_no_ret(t))
    }
}

pub fn example() {
    let list = new_list()
        .original_marker::<i32>()
        .returns(ReturnsInt(100))
        .returns(ReturnsInt(200))
        .returns(ReturnsStr("hello!"))
        .handle_result(())
        .handle_result(PrintMessage("handling results ..."))
        .returns(ReturnsDefault)
        .returns(ReturnsInt(300));

    let x = list.eval();

    println!("{x:?}"); // (100, 200, "hello!", 0, 300)

    let list2 = new_list()
        .original_marker::<Option<()>>()
        .returns(ReturnsDefault);

    let single = list2.eval();

    println!("{single:?}"); // None
}
