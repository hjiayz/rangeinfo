#![feature(inclusive_range,inclusive_range_syntax)]
use std::ops::{Range,RangeTo,RangeFrom,RangeFull,RangeInclusive,RangeToInclusive};
#[derive(PartialEq,Debug)]
pub enum RangeValue<T> {
    Open(T),
    Closed(T),
    Inf,
}
use self::RangeValue::{Open,Closed,Inf};
#[derive(PartialEq,Debug)]
pub struct RangeInfo <T>{
    start:RangeValue<T>,
    end:RangeValue<T>,
}
pub trait ToRangeInfo<T> {
    fn to_rangeinfo(&self)->RangeInfo<T>;
}
impl<T:Clone> ToRangeInfo<T> for Range<T>{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Closed(self.start.clone()),
            end:Open(self.end.clone()),
        }
    }
}
impl<T:Clone> ToRangeInfo<T> for RangeTo<T>{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Inf,
            end:Open(self.end.clone()),
        }
    }
}
impl<T:Clone> ToRangeInfo<T> for RangeFrom<T>{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Closed(self.start.clone()),
            end:Inf,
        }
    }
}
impl<T:Clone> ToRangeInfo<T> for RangeFull{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Inf,
            end:Inf,
        }
    }
}
impl<T:Clone> ToRangeInfo<T> for RangeInclusive<T>{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Closed(self.start.clone()),
            end:Closed(self.end.clone()),
        }
    }
}
impl<T:Clone> ToRangeInfo<T> for RangeToInclusive<T>{
    fn to_rangeinfo(&self)->RangeInfo<T>{
        RangeInfo{
            start:Inf,
            end:Closed(self.end.clone()),
        }
    }
}
#[test]
fn testall(){
    assert_eq!(RangeInfo { start: Closed(1), end: Open(2) },(1..2).to_rangeinfo() );
    assert_eq!(RangeInfo { start: Inf, end: Open(2) },( ..2).to_rangeinfo() );
    assert_eq!(RangeInfo { start: Closed(1), end: Inf },(1.. ).to_rangeinfo() );
    assert_eq!(RangeInfo::<i64> { start: Inf, end: Inf },( .. ).to_rangeinfo() );
    assert_eq!(RangeInfo { start: Closed(1), end: Closed(2) },(1...2).to_rangeinfo() );
    assert_eq!(RangeInfo { start: Inf, end: Closed(2) },( ...2).to_rangeinfo() );
}