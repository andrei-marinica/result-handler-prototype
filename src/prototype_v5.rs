use std::marker::PhantomData;

use crate::nested_tuples::{Flatten, NestedTupleAppend};

pub trait ReturnHandler<RBOriginal> {
    type Ret;

    fn single_return(&self) -> Self::Ret;
}

impl<RBOriginal> ReturnHandler<RBOriginal> for () {
    type Ret = ();

    fn single_return(&self) -> Self::Ret {}
}


pub struct PrintMessage(&'static str);

impl<RBOriginal> ReturnHandler<RBOriginal> for PrintMessage {
    type Ret = ();

    fn single_return(&self) -> Self::Ret {
        println!("{}", self.0)
    }
}

pub struct ReturnsStr(&'static str);

impl ReturnHandler<i32> for ReturnsStr {
    type Ret = &'static str;

    fn single_return(&self) -> Self::Ret {
        println!("returning str: {} ...", self.0);
        self.0
    }
}

pub struct ReturnsInt(i32);

impl ReturnHandler<i32> for ReturnsInt {
    type Ret = i32;

    fn single_return(&self) -> Self::Ret {
        println!("returning i32: {} ...", self.0);
        self.0
    }
}

pub struct ReturnsDefault;

impl<D: Default> ReturnHandler<D> for ReturnsDefault {
    type Ret = D;

    fn single_return(&self) -> Self::Ret {
        D::default()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub trait RetList {
    type Original;
    type ListReturn;

    fn list_return(&self) -> Self::ListReturn;
}

impl RetList for () {
    type Original = ();
    type ListReturn = ();

    fn list_return(&self) -> Self::ListReturn {}
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

pub struct ConsRet<Head, Tail>
where
    Head: ReturnHandler<Tail::Original>,
    Tail: RetList,
    Tail::ListReturn: NestedTupleAppend<<Head as ReturnHandler<Tail::Original>>::Ret>,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsRet<Head, Tail>
where
    Head: ReturnHandler<Tail::Original>,
    Tail: RetList,
    Tail::ListReturn: NestedTupleAppend<<Head as ReturnHandler<Tail::Original>>::Ret>,
{
    type Original = Tail::Original;
    type ListReturn = <Tail::ListReturn as NestedTupleAppend<
        <Head as ReturnHandler<Tail::Original>>::Ret,
    >>::Output;

    fn list_return(&self) -> Self::ListReturn {
        let tail_ret = self.tail.list_return();
        let head_ret = self.head.single_return();
        tail_ret.append(head_ret)
    }
}

/// Handlers that return nothing.
pub struct ConsNoRet<Head, Tail>
where
    Head: ReturnHandler<Tail::Original, Ret = ()>,
    Tail: RetList,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsNoRet<Head, Tail>
where
    Head: ReturnHandler<Tail::Original, Ret = ()>,
    Tail: RetList,
{
    type Original = Tail::Original;
    type ListReturn = Tail::ListReturn;

    fn list_return(&self) -> Self::ListReturn {
        self.head.single_return();
        self.tail.list_return()
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

    pub fn returns<Head>(self, rh: Head) -> ListWrapper<ConsRet<Head, L>>
    where
        Head: ReturnHandler<L::Original>,
        L::ListReturn: NestedTupleAppend<<Head as ReturnHandler<L::Original>>::Ret>,
    {
        ListWrapper(ConsRet {
            head: rh,
            tail: self.0,
        })
    }

    pub fn handle_result<Head>(self, rh: Head) -> ListWrapper<ConsNoRet<Head, L>>
    where
        Head: ReturnHandler<L::Original, Ret = ()>,
    {
        ListWrapper(ConsNoRet {
            head: rh,
            tail: self.0,
        })
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
