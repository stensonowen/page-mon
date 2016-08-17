#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use super::super::job;
use super::job::{ZERO, ONE, STAR};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Command {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::super::job;
    use super::super::job::{ZERO, ONE, STAR};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Command<
        'input,
    >(
        input: &'input str,
    ) -> Result<job::Command, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Command((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _28_22_2c_22_20_3cValue_3e_29((usize, job::Value, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2a((usize, ::std::vec::Vec<job::Value>, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2b((usize, ::std::vec::Vec<job::Value>, usize)),
        Action((usize, job::Action, usize)),
        Comma_3cValue_3e((usize, Vec<job::Value>, usize)),
        Command((usize, job::Command, usize)),
        Comment((usize, &'input str, usize)),
        ContVal((usize, job::ContVal, usize)),
        Contact((usize, job::Contact, usize)),
        DayOfWeek((usize, u8, usize)),
        Dir((usize, &'input str, usize)),
        Entry((usize, Vec<job::Value>, usize)),
        Entry__Day((usize, Vec<job::Value>, usize)),
        Entry__Month((usize, Vec<job::Value>, usize)),
        Line((usize, job::Line, usize)),
        MonthOfYear((usize, u8, usize)),
        Nickname((usize, job::Time, usize)),
        Num((usize, u8, usize)),
        Skip((usize, u8, usize)),
        Text((usize, &'input str, usize)),
        Time((usize, job::Time, usize)),
        Url((usize, &'input str, usize)),
        Value((usize, job::Value, usize)),
        Var((usize, job::Var, usize)),
        ____Command((usize, job::Command, usize)),
        ____Line((usize, job::Line, usize)),
        ____Var((usize, job::Var, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym0));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym0));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym0));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym0));
            }
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym0));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym0));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym0));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Command(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Nickname(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Time(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Command, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __nt = __Nonterminal::____Command((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::Time((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Action(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Url(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state27(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
        __sym1: &mut Option<(usize, job::Action, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __nt = __Nonterminal::Command((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __nt = __Nonterminal::Action((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60(input, __sym0);
                let __nt = __Nonterminal::Url((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state39(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state41(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state40(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym3));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym3));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state51(input, __tokens, __sym3));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym3));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym3));
            }
            Some((__loc1, (21, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym3));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state55(input, __tokens, __sym3));
            }
            Some((__loc1, (23, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state56(input, __tokens, __sym3));
            }
            Some((__loc1, (24, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state57(input, __tokens, __sym3));
            }
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state58(input, __tokens, __sym3));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state59(input, __tokens, __sym3));
            }
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state60(input, __tokens, __sym3));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state61(input, __tokens, __sym3));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state42(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state43(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state44(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Entry__Month(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state45(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::MonthOfYear(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state46(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state48(input, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state64(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state67(input, __tokens, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state68(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state69(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Contact(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state66(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state70(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state72(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state71(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state74(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state73(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31(input, __sym0);
                let __nt = __Nonterminal::Entry__Month((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym3: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state82(input, __tokens, __sym4));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state83(input, __tokens, __sym4));
            }
            Some((__loc1, (26, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state84(input, __tokens, __sym4));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state85(input, __tokens, __sym4));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym4));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym4));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state88(input, __tokens, __sym4));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym4));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state75(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::DayOfWeek(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state77(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state78(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Entry__Day(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state79(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state81(input, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32(input, __sym0);
                let __nt = __Nonterminal::Entry__Month((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state92(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action58(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state54<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state55<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state56<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state57<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state58<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state59<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state60<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state61<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state62<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state63<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state95(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state94(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state64<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state65<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state97(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state66<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Contact, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Action((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state67<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state68<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state69<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state70<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state71<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state72<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state73<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state74<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state99(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state98(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state75<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state76<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state101(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state100(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state77<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::Entry__Day((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state78<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::Entry__Day((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state79<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym3: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym4: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::Time((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state80<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state102(input, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state81<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state104(input, __tokens, __sym1));
            }
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state103(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state82<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state83<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state84<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state85<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state86<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state87<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state88<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state89<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state90<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state91<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state106(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state105(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state92<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state107(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state93<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state43(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state108(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state94<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state95<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state96<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state34(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state109(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state97<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state98<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state99<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state100<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state101<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state111(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state110(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state102<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state113(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state112(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state103<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state114(input, __tokens, __sym1, __sym2));
            }
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state104<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state82(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state115(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state105<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state106<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state107<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state43(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state116(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state108<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state109<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state110<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state111<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state112<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state113<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state114<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state82(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state90(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state76(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state80(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state117(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state115<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state116<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state117<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Command::parse_Command;

mod __parse__Line {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::super::job;
    use super::super::job::{ZERO, ONE, STAR};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Line<
        'input,
    >(
        input: &'input str,
    ) -> Result<job::Line, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Line((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _28_22_2c_22_20_3cValue_3e_29((usize, job::Value, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2a((usize, ::std::vec::Vec<job::Value>, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2b((usize, ::std::vec::Vec<job::Value>, usize)),
        Action((usize, job::Action, usize)),
        Comma_3cValue_3e((usize, Vec<job::Value>, usize)),
        Command((usize, job::Command, usize)),
        Comment((usize, &'input str, usize)),
        ContVal((usize, job::ContVal, usize)),
        Contact((usize, job::Contact, usize)),
        DayOfWeek((usize, u8, usize)),
        Dir((usize, &'input str, usize)),
        Entry((usize, Vec<job::Value>, usize)),
        Entry__Day((usize, Vec<job::Value>, usize)),
        Entry__Month((usize, Vec<job::Value>, usize)),
        Line((usize, job::Line, usize)),
        MonthOfYear((usize, u8, usize)),
        Nickname((usize, job::Time, usize)),
        Num((usize, u8, usize)),
        Skip((usize, u8, usize)),
        Text((usize, &'input str, usize)),
        Time((usize, job::Time, usize)),
        Url((usize, &'input str, usize)),
        Value((usize, job::Value, usize)),
        Var((usize, job::Var, usize)),
        ____Command((usize, job::Command, usize)),
        ____Line((usize, job::Line, usize)),
        ____Var((usize, job::Var, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym0));
            }
            Some((__loc1, (9, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym0));
            }
            Some((__loc1, (10, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(input, __tokens, __sym0));
            }
            Some((__loc1, (11, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym0));
            }
            Some((__loc1, (12, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state16(input, __tokens, __sym0));
            }
            Some((__loc1, (13, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym0));
            }
            Some((__loc1, (14, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym0));
            }
            Some((__loc1, (15, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym0));
            }
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym0));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state22(input, __tokens, __sym0));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym0));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym0));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym0));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Command(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Comment(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Line(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Nickname(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Time(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Command, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __nt = __Nonterminal::Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __nt = __Nonterminal::Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state28(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Line, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22(input, __sym0);
                let __nt = __Nonterminal::Time((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Action(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Url(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Var, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __nt = __Nonterminal::Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __nt = __Nonterminal::Comment((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(input, __sym0);
                let __nt = __Nonterminal::Nickname((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state36(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state38(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state39(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state40(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state41(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state43(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state42(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state44(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state45(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state46(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state48(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state52(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Time, usize)>,
        __sym1: &mut Option<(usize, job::Action, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __nt = __Nonterminal::Command((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state53(input, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(input, __sym0);
                let __nt = __Nonterminal::Action((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60(input, __sym0);
                let __nt = __Nonterminal::Url((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state55(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state57(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Dir(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state59(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state59(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state60(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state59(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state61(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state59(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state62(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state64(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state73(input, __tokens, __sym3));
            }
            Some((__loc1, (16, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state74(input, __tokens, __sym3));
            }
            Some((__loc1, (17, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state75(input, __tokens, __sym3));
            }
            Some((__loc1, (18, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state76(input, __tokens, __sym3));
            }
            Some((__loc1, (19, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state77(input, __tokens, __sym3));
            }
            Some((__loc1, (21, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state78(input, __tokens, __sym3));
            }
            Some((__loc1, (22, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state79(input, __tokens, __sym3));
            }
            Some((__loc1, (23, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state80(input, __tokens, __sym3));
            }
            Some((__loc1, (24, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state81(input, __tokens, __sym3));
            }
            Some((__loc1, (25, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state82(input, __tokens, __sym3));
            }
            Some((__loc1, (27, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state83(input, __tokens, __sym3));
            }
            Some((__loc1, (28, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state84(input, __tokens, __sym3));
            }
            Some((__loc1, (30, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state85(input, __tokens, __sym3));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state66(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state67(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state68(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Entry__Month(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state69(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::MonthOfYear(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state70(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state71(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state72(input, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state87(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state89(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state88(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (35, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state91(input, __tokens, __sym2));
            }
            Some((__loc1, (37, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state92(input, __tokens, __sym2));
            }
            Some((__loc1, (43, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state93(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Contact(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state90(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state54<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state94(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state55<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state56<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state57<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::Dir((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state58<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state59<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __nt = __Nonterminal::Text((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state60<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state61<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state62<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state63<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state96(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Url(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state95(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state64<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state65<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state98(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state97(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state66<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state67<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state100(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state99(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state68<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31(input, __sym0);
                let __nt = __Nonterminal::Entry__Month((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state69<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym3: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state108(input, __tokens, __sym4));
            }
            Some((__loc1, (20, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state109(input, __tokens, __sym4));
            }
            Some((__loc1, (26, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state110(input, __tokens, __sym4));
            }
            Some((__loc1, (29, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state111(input, __tokens, __sym4));
            }
            Some((__loc1, (31, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state112(input, __tokens, __sym4));
            }
            Some((__loc1, (32, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state113(input, __tokens, __sym4));
            }
            Some((__loc1, (33, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state114(input, __tokens, __sym4));
            }
            Some((__loc1, (34, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state115(input, __tokens, __sym4));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state116(input, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cValue_3e(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state101(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::ContVal(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state102(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::DayOfWeek(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state103(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Entry(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state104(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Entry__Day(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state105(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state106(input, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state107(input, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state70<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32(input, __sym0);
                let __nt = __Nonterminal::Entry__Month((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state71<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state117(input, __tokens, __sym0, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state72<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state119(input, __tokens, __sym1));
            }
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state118(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state73<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state74<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state75<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state76<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action58(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state77<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state78<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state79<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state80<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state81<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state82<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state83<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state84<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state85<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55(input, __sym0);
                let __nt = __Nonterminal::MonthOfYear((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state86<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state87<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state121(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state120(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state88<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state122(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state89<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state45(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state123(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state90<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Contact, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Action((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state91<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state92<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state93<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(input, __sym0);
                let __nt = __Nonterminal::Contact((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state94<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state95<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state124(input, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state96<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60(input, __sym0);
                let __nt = __Nonterminal::Url((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state97<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state98<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state99<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state100<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state126(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state125(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state101<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::Entry((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state102<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state128(input, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Skip(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state127(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state103<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(input, __sym0);
                let __nt = __Nonterminal::Entry__Day((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state104<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(input, __sym0);
                let __nt = __Nonterminal::Entry__Day((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state105<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym2: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym3: &mut Option<(usize, Vec<job::Value>, usize)>,
        __sym4: &mut Option<(usize, Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action21(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::Time((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state106<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state129(input, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34(input, __sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state107<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state131(input, __tokens, __sym1));
            }
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69(input, __sym0);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state130(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state108<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(input, __sym0);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state109<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state110<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state111<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state112<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state113<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state114<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state115<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(input, __sym0);
                let __nt = __Nonterminal::DayOfWeek((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state116<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state117<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state133(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state132(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state118<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state134(input, __tokens, __sym1, __sym2));
            }
            Some((_, (0, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state119<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state73(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state67(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state71(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state135(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state120<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state121<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state122<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state49(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state50(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state45(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state136(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state123<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state124<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
        __sym4: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state125<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state126<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state127<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::ContVal, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state128<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state138(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state137(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state129<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state140(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state139(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state130<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Value, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state141(input, __tokens, __sym1, __sym2));
            }
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action70(input, __sym0, __sym1);
                let __nt = __Nonterminal::Comma_3cValue_3e((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state131<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state108(input, __tokens, __sym1));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state116(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state102(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state106(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state142(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state132<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state133<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state134<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state73(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state86(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state67(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state71(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state143(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state135<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state136<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (16, _), _)) |
            Some((_, (17, _), _)) |
            Some((_, (18, _), _)) |
            Some((_, (19, _), _)) |
            Some((_, (21, _), _)) |
            Some((_, (22, _), _)) |
            Some((_, (23, _), _)) |
            Some((_, (24, _), _)) |
            Some((_, (25, _), _)) |
            Some((_, (27, _), _)) |
            Some((_, (28, _), _)) |
            Some((_, (30, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state137<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1);
                let __nt = __Nonterminal::Skip((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state138<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state139<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u8, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, u8, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ContVal((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state140<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59(input, __sym0);
                let __nt = __Nonterminal::Num((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state141<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state108(input, __tokens, __sym2));
            }
            Some((__loc1, (44, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state116(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ContVal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state102(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state106(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state144(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state142<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67(input, __sym0, __sym1);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state143<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (20, _), _)) |
            Some((_, (26, _), _)) |
            Some((_, (29, _), _)) |
            Some((_, (31, _), _)) |
            Some((_, (32, _), _)) |
            Some((_, (33, _), _)) |
            Some((_, (34, _), _)) |
            Some((_, (44, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state144<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<job::Value>, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, job::Value, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (45, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action68(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::_28_22_2c_22_20_3cValue_3e_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Line::parse_Line;

mod __parse__Var {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::super::job;
    use super::super::job::{ZERO, ONE, STAR};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Var<
        'input,
    >(
        input: &'input str,
    ) -> Result<job::Var, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Var((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _28_22_2c_22_20_3cValue_3e_29((usize, job::Value, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2a((usize, ::std::vec::Vec<job::Value>, usize)),
        _28_22_2c_22_20_3cValue_3e_29_2b((usize, ::std::vec::Vec<job::Value>, usize)),
        Action((usize, job::Action, usize)),
        Comma_3cValue_3e((usize, Vec<job::Value>, usize)),
        Command((usize, job::Command, usize)),
        Comment((usize, &'input str, usize)),
        ContVal((usize, job::ContVal, usize)),
        Contact((usize, job::Contact, usize)),
        DayOfWeek((usize, u8, usize)),
        Dir((usize, &'input str, usize)),
        Entry((usize, Vec<job::Value>, usize)),
        Entry__Day((usize, Vec<job::Value>, usize)),
        Entry__Month((usize, Vec<job::Value>, usize)),
        Line((usize, job::Line, usize)),
        MonthOfYear((usize, u8, usize)),
        Nickname((usize, job::Time, usize)),
        Num((usize, u8, usize)),
        Skip((usize, u8, usize)),
        Text((usize, &'input str, usize)),
        Time((usize, job::Time, usize)),
        Url((usize, &'input str, usize)),
        Value((usize, job::Value, usize)),
        Var((usize, job::Var, usize)),
        ____Command((usize, job::Command, usize)),
        ____Line((usize, job::Line, usize)),
        ____Var((usize, job::Var, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (36, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state2(input, __tokens, __sym0));
            }
            Some((__loc1, (38, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(input, __tokens, __sym0));
            }
            Some((__loc1, (39, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0));
            }
            Some((__loc1, (40, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state5(input, __tokens, __sym0));
            }
            Some((__loc1, (41, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state6(input, __tokens, __sym0));
            }
            Some((__loc1, (42, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Var(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, job::Var, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __nt = __Nonterminal::____Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state13(input, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Dir(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (8, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state17(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Text(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::Dir((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __nt = __Nonterminal::Text((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (45, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Url(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60(input, __sym0);
                let __nt = __Nonterminal::Url((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
        __sym4: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::Var((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Var::parse_Var;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        34 => /* '\"' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        35 => /* '#' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((44, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        64 => /* '@' */ {
                            __current_state = 9;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_state = 10;
                            continue;
                        }
                        68 => /* 'D' */ {
                            __current_state = 11;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_state = 12;
                            continue;
                        }
                        70 => /* 'F' */ {
                            __current_state = 13;
                            continue;
                        }
                        74 => /* 'J' */ {
                            __current_state = 14;
                            continue;
                        }
                        77 => /* 'M' */ {
                            __current_state = 15;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_state = 16;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_state = 17;
                            continue;
                        }
                        80 => /* 'P' */ {
                            __current_state = 18;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_state = 19;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_state = 20;
                            continue;
                        }
                        87 => /* 'W' */ {
                            __current_state = 21;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 10;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_state = 11;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 12;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_state = 13;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 22;
                            continue;
                        }
                        106 => /* 'j' */ {
                            __current_state = 14;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 15;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 16;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 17;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 18;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 19;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 20;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_state = 24;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_state = 25;
                            continue;
                        }
                        48 ... 57 => {
                            __current_state = 24;
                            continue;
                        }
                        64 ... 90 => {
                            __current_state = 24;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_state = 24;
                            continue;
                        }
                        97 ... 122 => {
                            __current_state = 24;
                            continue;
                        }
                        170 => /* '\u{aa}' */ {
                            __current_state = 24;
                            continue;
                        }
                        181 => /* '\u{b5}' */ {
                            __current_state = 24;
                            continue;
                        }
                        186 => /* '\u{ba}' */ {
                            __current_state = 24;
                            continue;
                        }
                        192 ... 214 => {
                            __current_state = 24;
                            continue;
                        }
                        216 ... 246 => {
                            __current_state = 24;
                            continue;
                        }
                        248 ... 705 => {
                            __current_state = 24;
                            continue;
                        }
                        710 ... 721 => {
                            __current_state = 24;
                            continue;
                        }
                        736 ... 740 => {
                            __current_state = 24;
                            continue;
                        }
                        748 => /* '\u{2ec}' */ {
                            __current_state = 24;
                            continue;
                        }
                        750 => /* '\u{2ee}' */ {
                            __current_state = 24;
                            continue;
                        }
                        768 ... 884 => {
                            __current_state = 24;
                            continue;
                        }
                        886 ... 887 => {
                            __current_state = 24;
                            continue;
                        }
                        890 ... 893 => {
                            __current_state = 24;
                            continue;
                        }
                        895 => /* '\u{37f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        902 => /* '\u{386}' */ {
                            __current_state = 24;
                            continue;
                        }
                        904 ... 906 => {
                            __current_state = 24;
                            continue;
                        }
                        908 => /* '\u{38c}' */ {
                            __current_state = 24;
                            continue;
                        }
                        910 ... 929 => {
                            __current_state = 24;
                            continue;
                        }
                        931 ... 1013 => {
                            __current_state = 24;
                            continue;
                        }
                        1015 ... 1153 => {
                            __current_state = 24;
                            continue;
                        }
                        1155 ... 1327 => {
                            __current_state = 24;
                            continue;
                        }
                        1329 ... 1366 => {
                            __current_state = 24;
                            continue;
                        }
                        1369 => /* '\u{559}' */ {
                            __current_state = 24;
                            continue;
                        }
                        1377 ... 1415 => {
                            __current_state = 24;
                            continue;
                        }
                        1425 ... 1469 => {
                            __current_state = 24;
                            continue;
                        }
                        1471 => /* '\u{5bf}' */ {
                            __current_state = 24;
                            continue;
                        }
                        1473 ... 1474 => {
                            __current_state = 24;
                            continue;
                        }
                        1476 ... 1477 => {
                            __current_state = 24;
                            continue;
                        }
                        1479 => /* '\u{5c7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        1488 ... 1514 => {
                            __current_state = 24;
                            continue;
                        }
                        1520 ... 1522 => {
                            __current_state = 24;
                            continue;
                        }
                        1552 ... 1562 => {
                            __current_state = 24;
                            continue;
                        }
                        1568 ... 1641 => {
                            __current_state = 24;
                            continue;
                        }
                        1646 ... 1747 => {
                            __current_state = 24;
                            continue;
                        }
                        1749 ... 1756 => {
                            __current_state = 24;
                            continue;
                        }
                        1759 ... 1768 => {
                            __current_state = 24;
                            continue;
                        }
                        1770 ... 1788 => {
                            __current_state = 24;
                            continue;
                        }
                        1791 => /* '\u{6ff}' */ {
                            __current_state = 24;
                            continue;
                        }
                        1808 ... 1866 => {
                            __current_state = 24;
                            continue;
                        }
                        1869 ... 1969 => {
                            __current_state = 24;
                            continue;
                        }
                        1984 ... 2037 => {
                            __current_state = 24;
                            continue;
                        }
                        2042 => /* '\u{7fa}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2048 ... 2093 => {
                            __current_state = 24;
                            continue;
                        }
                        2112 ... 2139 => {
                            __current_state = 24;
                            continue;
                        }
                        2208 ... 2228 => {
                            __current_state = 24;
                            continue;
                        }
                        2275 ... 2403 => {
                            __current_state = 24;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_state = 24;
                            continue;
                        }
                        2417 ... 2435 => {
                            __current_state = 24;
                            continue;
                        }
                        2437 ... 2444 => {
                            __current_state = 24;
                            continue;
                        }
                        2447 ... 2448 => {
                            __current_state = 24;
                            continue;
                        }
                        2451 ... 2472 => {
                            __current_state = 24;
                            continue;
                        }
                        2474 ... 2480 => {
                            __current_state = 24;
                            continue;
                        }
                        2482 => /* '\u{9b2}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2486 ... 2489 => {
                            __current_state = 24;
                            continue;
                        }
                        2492 ... 2500 => {
                            __current_state = 24;
                            continue;
                        }
                        2503 ... 2504 => {
                            __current_state = 24;
                            continue;
                        }
                        2507 ... 2510 => {
                            __current_state = 24;
                            continue;
                        }
                        2519 => /* '\u{9d7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2524 ... 2525 => {
                            __current_state = 24;
                            continue;
                        }
                        2527 ... 2531 => {
                            __current_state = 24;
                            continue;
                        }
                        2534 ... 2545 => {
                            __current_state = 24;
                            continue;
                        }
                        2561 ... 2563 => {
                            __current_state = 24;
                            continue;
                        }
                        2565 ... 2570 => {
                            __current_state = 24;
                            continue;
                        }
                        2575 ... 2576 => {
                            __current_state = 24;
                            continue;
                        }
                        2579 ... 2600 => {
                            __current_state = 24;
                            continue;
                        }
                        2602 ... 2608 => {
                            __current_state = 24;
                            continue;
                        }
                        2610 ... 2611 => {
                            __current_state = 24;
                            continue;
                        }
                        2613 ... 2614 => {
                            __current_state = 24;
                            continue;
                        }
                        2616 ... 2617 => {
                            __current_state = 24;
                            continue;
                        }
                        2620 => /* '\u{a3c}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2622 ... 2626 => {
                            __current_state = 24;
                            continue;
                        }
                        2631 ... 2632 => {
                            __current_state = 24;
                            continue;
                        }
                        2635 ... 2637 => {
                            __current_state = 24;
                            continue;
                        }
                        2641 => /* '\u{a51}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2649 ... 2652 => {
                            __current_state = 24;
                            continue;
                        }
                        2654 => /* '\u{a5e}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2662 ... 2677 => {
                            __current_state = 24;
                            continue;
                        }
                        2689 ... 2691 => {
                            __current_state = 24;
                            continue;
                        }
                        2693 ... 2701 => {
                            __current_state = 24;
                            continue;
                        }
                        2703 ... 2705 => {
                            __current_state = 24;
                            continue;
                        }
                        2707 ... 2728 => {
                            __current_state = 24;
                            continue;
                        }
                        2730 ... 2736 => {
                            __current_state = 24;
                            continue;
                        }
                        2738 ... 2739 => {
                            __current_state = 24;
                            continue;
                        }
                        2741 ... 2745 => {
                            __current_state = 24;
                            continue;
                        }
                        2748 ... 2757 => {
                            __current_state = 24;
                            continue;
                        }
                        2759 ... 2761 => {
                            __current_state = 24;
                            continue;
                        }
                        2763 ... 2765 => {
                            __current_state = 24;
                            continue;
                        }
                        2768 => /* '\u{ad0}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2784 ... 2787 => {
                            __current_state = 24;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_state = 24;
                            continue;
                        }
                        2809 => /* '\u{af9}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2817 ... 2819 => {
                            __current_state = 24;
                            continue;
                        }
                        2821 ... 2828 => {
                            __current_state = 24;
                            continue;
                        }
                        2831 ... 2832 => {
                            __current_state = 24;
                            continue;
                        }
                        2835 ... 2856 => {
                            __current_state = 24;
                            continue;
                        }
                        2858 ... 2864 => {
                            __current_state = 24;
                            continue;
                        }
                        2866 ... 2867 => {
                            __current_state = 24;
                            continue;
                        }
                        2869 ... 2873 => {
                            __current_state = 24;
                            continue;
                        }
                        2876 ... 2884 => {
                            __current_state = 24;
                            continue;
                        }
                        2887 ... 2888 => {
                            __current_state = 24;
                            continue;
                        }
                        2891 ... 2893 => {
                            __current_state = 24;
                            continue;
                        }
                        2902 ... 2903 => {
                            __current_state = 24;
                            continue;
                        }
                        2908 ... 2909 => {
                            __current_state = 24;
                            continue;
                        }
                        2911 ... 2915 => {
                            __current_state = 24;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_state = 24;
                            continue;
                        }
                        2929 => /* '\u{b71}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2946 ... 2947 => {
                            __current_state = 24;
                            continue;
                        }
                        2949 ... 2954 => {
                            __current_state = 24;
                            continue;
                        }
                        2958 ... 2960 => {
                            __current_state = 24;
                            continue;
                        }
                        2962 ... 2965 => {
                            __current_state = 24;
                            continue;
                        }
                        2969 ... 2970 => {
                            __current_state = 24;
                            continue;
                        }
                        2972 => /* '\u{b9c}' */ {
                            __current_state = 24;
                            continue;
                        }
                        2974 ... 2975 => {
                            __current_state = 24;
                            continue;
                        }
                        2979 ... 2980 => {
                            __current_state = 24;
                            continue;
                        }
                        2984 ... 2986 => {
                            __current_state = 24;
                            continue;
                        }
                        2990 ... 3001 => {
                            __current_state = 24;
                            continue;
                        }
                        3006 ... 3010 => {
                            __current_state = 24;
                            continue;
                        }
                        3014 ... 3016 => {
                            __current_state = 24;
                            continue;
                        }
                        3018 ... 3021 => {
                            __current_state = 24;
                            continue;
                        }
                        3024 => /* '\u{bd0}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3031 => /* '\u{bd7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_state = 24;
                            continue;
                        }
                        3072 ... 3075 => {
                            __current_state = 24;
                            continue;
                        }
                        3077 ... 3084 => {
                            __current_state = 24;
                            continue;
                        }
                        3086 ... 3088 => {
                            __current_state = 24;
                            continue;
                        }
                        3090 ... 3112 => {
                            __current_state = 24;
                            continue;
                        }
                        3114 ... 3129 => {
                            __current_state = 24;
                            continue;
                        }
                        3133 ... 3140 => {
                            __current_state = 24;
                            continue;
                        }
                        3142 ... 3144 => {
                            __current_state = 24;
                            continue;
                        }
                        3146 ... 3149 => {
                            __current_state = 24;
                            continue;
                        }
                        3157 ... 3158 => {
                            __current_state = 24;
                            continue;
                        }
                        3160 ... 3162 => {
                            __current_state = 24;
                            continue;
                        }
                        3168 ... 3171 => {
                            __current_state = 24;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_state = 24;
                            continue;
                        }
                        3201 ... 3203 => {
                            __current_state = 24;
                            continue;
                        }
                        3205 ... 3212 => {
                            __current_state = 24;
                            continue;
                        }
                        3214 ... 3216 => {
                            __current_state = 24;
                            continue;
                        }
                        3218 ... 3240 => {
                            __current_state = 24;
                            continue;
                        }
                        3242 ... 3251 => {
                            __current_state = 24;
                            continue;
                        }
                        3253 ... 3257 => {
                            __current_state = 24;
                            continue;
                        }
                        3260 ... 3268 => {
                            __current_state = 24;
                            continue;
                        }
                        3270 ... 3272 => {
                            __current_state = 24;
                            continue;
                        }
                        3274 ... 3277 => {
                            __current_state = 24;
                            continue;
                        }
                        3285 ... 3286 => {
                            __current_state = 24;
                            continue;
                        }
                        3294 => /* '\u{cde}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3296 ... 3299 => {
                            __current_state = 24;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_state = 24;
                            continue;
                        }
                        3313 ... 3314 => {
                            __current_state = 24;
                            continue;
                        }
                        3329 ... 3331 => {
                            __current_state = 24;
                            continue;
                        }
                        3333 ... 3340 => {
                            __current_state = 24;
                            continue;
                        }
                        3342 ... 3344 => {
                            __current_state = 24;
                            continue;
                        }
                        3346 ... 3386 => {
                            __current_state = 24;
                            continue;
                        }
                        3389 ... 3396 => {
                            __current_state = 24;
                            continue;
                        }
                        3398 ... 3400 => {
                            __current_state = 24;
                            continue;
                        }
                        3402 ... 3406 => {
                            __current_state = 24;
                            continue;
                        }
                        3415 => /* '\u{d57}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3423 ... 3427 => {
                            __current_state = 24;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_state = 24;
                            continue;
                        }
                        3450 ... 3455 => {
                            __current_state = 24;
                            continue;
                        }
                        3458 ... 3459 => {
                            __current_state = 24;
                            continue;
                        }
                        3461 ... 3478 => {
                            __current_state = 24;
                            continue;
                        }
                        3482 ... 3505 => {
                            __current_state = 24;
                            continue;
                        }
                        3507 ... 3515 => {
                            __current_state = 24;
                            continue;
                        }
                        3517 => /* '\u{dbd}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3520 ... 3526 => {
                            __current_state = 24;
                            continue;
                        }
                        3530 => /* '\u{dca}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3535 ... 3540 => {
                            __current_state = 24;
                            continue;
                        }
                        3542 => /* '\u{dd6}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3544 ... 3551 => {
                            __current_state = 24;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_state = 24;
                            continue;
                        }
                        3570 ... 3571 => {
                            __current_state = 24;
                            continue;
                        }
                        3585 ... 3642 => {
                            __current_state = 24;
                            continue;
                        }
                        3648 ... 3662 => {
                            __current_state = 24;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_state = 24;
                            continue;
                        }
                        3713 ... 3714 => {
                            __current_state = 24;
                            continue;
                        }
                        3716 => /* '\u{e84}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3719 ... 3720 => {
                            __current_state = 24;
                            continue;
                        }
                        3722 => /* '\u{e8a}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3725 => /* '\u{e8d}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3732 ... 3735 => {
                            __current_state = 24;
                            continue;
                        }
                        3737 ... 3743 => {
                            __current_state = 24;
                            continue;
                        }
                        3745 ... 3747 => {
                            __current_state = 24;
                            continue;
                        }
                        3749 => /* '\u{ea5}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3751 => /* '\u{ea7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3754 ... 3755 => {
                            __current_state = 24;
                            continue;
                        }
                        3757 ... 3769 => {
                            __current_state = 24;
                            continue;
                        }
                        3771 ... 3773 => {
                            __current_state = 24;
                            continue;
                        }
                        3776 ... 3780 => {
                            __current_state = 24;
                            continue;
                        }
                        3782 => /* '\u{ec6}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3784 ... 3789 => {
                            __current_state = 24;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_state = 24;
                            continue;
                        }
                        3804 ... 3807 => {
                            __current_state = 24;
                            continue;
                        }
                        3840 => /* '\u{f00}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3864 ... 3865 => {
                            __current_state = 24;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_state = 24;
                            continue;
                        }
                        3893 => /* '\u{f35}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3895 => /* '\u{f37}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3897 => /* '\u{f39}' */ {
                            __current_state = 24;
                            continue;
                        }
                        3902 ... 3911 => {
                            __current_state = 24;
                            continue;
                        }
                        3913 ... 3948 => {
                            __current_state = 24;
                            continue;
                        }
                        3953 ... 3972 => {
                            __current_state = 24;
                            continue;
                        }
                        3974 ... 3991 => {
                            __current_state = 24;
                            continue;
                        }
                        3993 ... 4028 => {
                            __current_state = 24;
                            continue;
                        }
                        4038 => /* '\u{fc6}' */ {
                            __current_state = 24;
                            continue;
                        }
                        4096 ... 4169 => {
                            __current_state = 24;
                            continue;
                        }
                        4176 ... 4253 => {
                            __current_state = 24;
                            continue;
                        }
                        4256 ... 4293 => {
                            __current_state = 24;
                            continue;
                        }
                        4295 => /* '\u{10c7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        4301 => /* '\u{10cd}' */ {
                            __current_state = 24;
                            continue;
                        }
                        4304 ... 4346 => {
                            __current_state = 24;
                            continue;
                        }
                        4348 ... 4680 => {
                            __current_state = 24;
                            continue;
                        }
                        4682 ... 4685 => {
                            __current_state = 24;
                            continue;
                        }
                        4688 ... 4694 => {
                            __current_state = 24;
                            continue;
                        }
                        4696 => /* '\u{1258}' */ {
                            __current_state = 24;
                            continue;
                        }
                        4698 ... 4701 => {
                            __current_state = 24;
                            continue;
                        }
                        4704 ... 4744 => {
                            __current_state = 24;
                            continue;
                        }
                        4746 ... 4749 => {
                            __current_state = 24;
                            continue;
                        }
                        4752 ... 4784 => {
                            __current_state = 24;
                            continue;
                        }
                        4786 ... 4789 => {
                            __current_state = 24;
                            continue;
                        }
                        4792 ... 4798 => {
                            __current_state = 24;
                            continue;
                        }
                        4800 => /* '\u{12c0}' */ {
                            __current_state = 24;
                            continue;
                        }
                        4802 ... 4805 => {
                            __current_state = 24;
                            continue;
                        }
                        4808 ... 4822 => {
                            __current_state = 24;
                            continue;
                        }
                        4824 ... 4880 => {
                            __current_state = 24;
                            continue;
                        }
                        4882 ... 4885 => {
                            __current_state = 24;
                            continue;
                        }
                        4888 ... 4954 => {
                            __current_state = 24;
                            continue;
                        }
                        4957 ... 4959 => {
                            __current_state = 24;
                            continue;
                        }
                        4992 ... 5007 => {
                            __current_state = 24;
                            continue;
                        }
                        5024 ... 5109 => {
                            __current_state = 24;
                            continue;
                        }
                        5112 ... 5117 => {
                            __current_state = 24;
                            continue;
                        }
                        5121 ... 5740 => {
                            __current_state = 24;
                            continue;
                        }
                        5743 ... 5759 => {
                            __current_state = 24;
                            continue;
                        }
                        5761 ... 5786 => {
                            __current_state = 24;
                            continue;
                        }
                        5792 ... 5866 => {
                            __current_state = 24;
                            continue;
                        }
                        5870 ... 5880 => {
                            __current_state = 24;
                            continue;
                        }
                        5888 ... 5900 => {
                            __current_state = 24;
                            continue;
                        }
                        5902 ... 5908 => {
                            __current_state = 24;
                            continue;
                        }
                        5920 ... 5940 => {
                            __current_state = 24;
                            continue;
                        }
                        5952 ... 5971 => {
                            __current_state = 24;
                            continue;
                        }
                        5984 ... 5996 => {
                            __current_state = 24;
                            continue;
                        }
                        5998 ... 6000 => {
                            __current_state = 24;
                            continue;
                        }
                        6002 ... 6003 => {
                            __current_state = 24;
                            continue;
                        }
                        6016 ... 6099 => {
                            __current_state = 24;
                            continue;
                        }
                        6103 => /* '\u{17d7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        6108 ... 6109 => {
                            __current_state = 24;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_state = 24;
                            continue;
                        }
                        6155 ... 6157 => {
                            __current_state = 24;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_state = 24;
                            continue;
                        }
                        6176 ... 6263 => {
                            __current_state = 24;
                            continue;
                        }
                        6272 ... 6314 => {
                            __current_state = 24;
                            continue;
                        }
                        6320 ... 6389 => {
                            __current_state = 24;
                            continue;
                        }
                        6400 ... 6430 => {
                            __current_state = 24;
                            continue;
                        }
                        6432 ... 6443 => {
                            __current_state = 24;
                            continue;
                        }
                        6448 ... 6459 => {
                            __current_state = 24;
                            continue;
                        }
                        6470 ... 6509 => {
                            __current_state = 24;
                            continue;
                        }
                        6512 ... 6516 => {
                            __current_state = 24;
                            continue;
                        }
                        6528 ... 6571 => {
                            __current_state = 24;
                            continue;
                        }
                        6576 ... 6601 => {
                            __current_state = 24;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_state = 24;
                            continue;
                        }
                        6656 ... 6683 => {
                            __current_state = 24;
                            continue;
                        }
                        6688 ... 6750 => {
                            __current_state = 24;
                            continue;
                        }
                        6752 ... 6780 => {
                            __current_state = 24;
                            continue;
                        }
                        6783 ... 6793 => {
                            __current_state = 24;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_state = 24;
                            continue;
                        }
                        6823 => /* '\u{1aa7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        6832 ... 6846 => {
                            __current_state = 24;
                            continue;
                        }
                        6912 ... 6987 => {
                            __current_state = 24;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_state = 24;
                            continue;
                        }
                        7019 ... 7027 => {
                            __current_state = 24;
                            continue;
                        }
                        7040 ... 7155 => {
                            __current_state = 24;
                            continue;
                        }
                        7168 ... 7223 => {
                            __current_state = 24;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_state = 24;
                            continue;
                        }
                        7245 ... 7293 => {
                            __current_state = 24;
                            continue;
                        }
                        7376 ... 7378 => {
                            __current_state = 24;
                            continue;
                        }
                        7380 ... 7414 => {
                            __current_state = 24;
                            continue;
                        }
                        7416 ... 7417 => {
                            __current_state = 24;
                            continue;
                        }
                        7424 ... 7669 => {
                            __current_state = 24;
                            continue;
                        }
                        7676 ... 7957 => {
                            __current_state = 24;
                            continue;
                        }
                        7960 ... 7965 => {
                            __current_state = 24;
                            continue;
                        }
                        7968 ... 8005 => {
                            __current_state = 24;
                            continue;
                        }
                        8008 ... 8013 => {
                            __current_state = 24;
                            continue;
                        }
                        8016 ... 8023 => {
                            __current_state = 24;
                            continue;
                        }
                        8025 => /* '\u{1f59}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8027 => /* '\u{1f5b}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8029 => /* '\u{1f5d}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8031 ... 8061 => {
                            __current_state = 24;
                            continue;
                        }
                        8064 ... 8116 => {
                            __current_state = 24;
                            continue;
                        }
                        8118 ... 8124 => {
                            __current_state = 24;
                            continue;
                        }
                        8126 => /* '\u{1fbe}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8130 ... 8132 => {
                            __current_state = 24;
                            continue;
                        }
                        8134 ... 8140 => {
                            __current_state = 24;
                            continue;
                        }
                        8144 ... 8147 => {
                            __current_state = 24;
                            continue;
                        }
                        8150 ... 8155 => {
                            __current_state = 24;
                            continue;
                        }
                        8160 ... 8172 => {
                            __current_state = 24;
                            continue;
                        }
                        8178 ... 8180 => {
                            __current_state = 24;
                            continue;
                        }
                        8182 ... 8188 => {
                            __current_state = 24;
                            continue;
                        }
                        8204 ... 8205 => {
                            __current_state = 24;
                            continue;
                        }
                        8255 ... 8256 => {
                            __current_state = 24;
                            continue;
                        }
                        8276 => /* '\u{2054}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8305 => /* '\u{2071}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8319 => /* '\u{207f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8336 ... 8348 => {
                            __current_state = 24;
                            continue;
                        }
                        8400 ... 8432 => {
                            __current_state = 24;
                            continue;
                        }
                        8450 => /* '\u{2102}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8455 => /* '\u{2107}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8458 ... 8467 => {
                            __current_state = 24;
                            continue;
                        }
                        8469 => /* '\u{2115}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8473 ... 8477 => {
                            __current_state = 24;
                            continue;
                        }
                        8484 => /* '\u{2124}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8486 => /* '\u{2126}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8488 => /* '\u{2128}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8490 ... 8493 => {
                            __current_state = 24;
                            continue;
                        }
                        8495 ... 8505 => {
                            __current_state = 24;
                            continue;
                        }
                        8508 ... 8511 => {
                            __current_state = 24;
                            continue;
                        }
                        8517 ... 8521 => {
                            __current_state = 24;
                            continue;
                        }
                        8526 => /* '\u{214e}' */ {
                            __current_state = 24;
                            continue;
                        }
                        8544 ... 8584 => {
                            __current_state = 24;
                            continue;
                        }
                        9398 ... 9449 => {
                            __current_state = 24;
                            continue;
                        }
                        11264 ... 11310 => {
                            __current_state = 24;
                            continue;
                        }
                        11312 ... 11358 => {
                            __current_state = 24;
                            continue;
                        }
                        11360 ... 11492 => {
                            __current_state = 24;
                            continue;
                        }
                        11499 ... 11507 => {
                            __current_state = 24;
                            continue;
                        }
                        11520 ... 11557 => {
                            __current_state = 24;
                            continue;
                        }
                        11559 => /* '\u{2d27}' */ {
                            __current_state = 24;
                            continue;
                        }
                        11565 => /* '\u{2d2d}' */ {
                            __current_state = 24;
                            continue;
                        }
                        11568 ... 11623 => {
                            __current_state = 24;
                            continue;
                        }
                        11631 => /* '\u{2d6f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        11647 ... 11670 => {
                            __current_state = 24;
                            continue;
                        }
                        11680 ... 11686 => {
                            __current_state = 24;
                            continue;
                        }
                        11688 ... 11694 => {
                            __current_state = 24;
                            continue;
                        }
                        11696 ... 11702 => {
                            __current_state = 24;
                            continue;
                        }
                        11704 ... 11710 => {
                            __current_state = 24;
                            continue;
                        }
                        11712 ... 11718 => {
                            __current_state = 24;
                            continue;
                        }
                        11720 ... 11726 => {
                            __current_state = 24;
                            continue;
                        }
                        11728 ... 11734 => {
                            __current_state = 24;
                            continue;
                        }
                        11736 ... 11742 => {
                            __current_state = 24;
                            continue;
                        }
                        11744 ... 11775 => {
                            __current_state = 24;
                            continue;
                        }
                        11823 => /* '\u{2e2f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        12293 ... 12295 => {
                            __current_state = 24;
                            continue;
                        }
                        12321 ... 12335 => {
                            __current_state = 24;
                            continue;
                        }
                        12337 ... 12341 => {
                            __current_state = 24;
                            continue;
                        }
                        12344 ... 12348 => {
                            __current_state = 24;
                            continue;
                        }
                        12353 ... 12438 => {
                            __current_state = 24;
                            continue;
                        }
                        12441 ... 12442 => {
                            __current_state = 24;
                            continue;
                        }
                        12445 ... 12447 => {
                            __current_state = 24;
                            continue;
                        }
                        12449 ... 12538 => {
                            __current_state = 24;
                            continue;
                        }
                        12540 ... 12543 => {
                            __current_state = 24;
                            continue;
                        }
                        12549 ... 12589 => {
                            __current_state = 24;
                            continue;
                        }
                        12593 ... 12686 => {
                            __current_state = 24;
                            continue;
                        }
                        12704 ... 12730 => {
                            __current_state = 24;
                            continue;
                        }
                        12784 ... 12799 => {
                            __current_state = 24;
                            continue;
                        }
                        13312 ... 19893 => {
                            __current_state = 24;
                            continue;
                        }
                        19968 ... 40917 => {
                            __current_state = 24;
                            continue;
                        }
                        40960 ... 42124 => {
                            __current_state = 24;
                            continue;
                        }
                        42192 ... 42237 => {
                            __current_state = 24;
                            continue;
                        }
                        42240 ... 42508 => {
                            __current_state = 24;
                            continue;
                        }
                        42512 ... 42539 => {
                            __current_state = 24;
                            continue;
                        }
                        42560 ... 42610 => {
                            __current_state = 24;
                            continue;
                        }
                        42612 ... 42621 => {
                            __current_state = 24;
                            continue;
                        }
                        42623 ... 42737 => {
                            __current_state = 24;
                            continue;
                        }
                        42775 ... 42783 => {
                            __current_state = 24;
                            continue;
                        }
                        42786 ... 42888 => {
                            __current_state = 24;
                            continue;
                        }
                        42891 ... 42925 => {
                            __current_state = 24;
                            continue;
                        }
                        42928 ... 42935 => {
                            __current_state = 24;
                            continue;
                        }
                        42999 ... 43047 => {
                            __current_state = 24;
                            continue;
                        }
                        43072 ... 43123 => {
                            __current_state = 24;
                            continue;
                        }
                        43136 ... 43204 => {
                            __current_state = 24;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_state = 24;
                            continue;
                        }
                        43232 ... 43255 => {
                            __current_state = 24;
                            continue;
                        }
                        43259 => /* '\u{a8fb}' */ {
                            __current_state = 24;
                            continue;
                        }
                        43261 => /* '\u{a8fd}' */ {
                            __current_state = 24;
                            continue;
                        }
                        43264 ... 43309 => {
                            __current_state = 24;
                            continue;
                        }
                        43312 ... 43347 => {
                            __current_state = 24;
                            continue;
                        }
                        43360 ... 43388 => {
                            __current_state = 24;
                            continue;
                        }
                        43392 ... 43456 => {
                            __current_state = 24;
                            continue;
                        }
                        43471 ... 43481 => {
                            __current_state = 24;
                            continue;
                        }
                        43488 ... 43518 => {
                            __current_state = 24;
                            continue;
                        }
                        43520 ... 43574 => {
                            __current_state = 24;
                            continue;
                        }
                        43584 ... 43597 => {
                            __current_state = 24;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_state = 24;
                            continue;
                        }
                        43616 ... 43638 => {
                            __current_state = 24;
                            continue;
                        }
                        43642 ... 43714 => {
                            __current_state = 24;
                            continue;
                        }
                        43739 ... 43741 => {
                            __current_state = 24;
                            continue;
                        }
                        43744 ... 43759 => {
                            __current_state = 24;
                            continue;
                        }
                        43762 ... 43766 => {
                            __current_state = 24;
                            continue;
                        }
                        43777 ... 43782 => {
                            __current_state = 24;
                            continue;
                        }
                        43785 ... 43790 => {
                            __current_state = 24;
                            continue;
                        }
                        43793 ... 43798 => {
                            __current_state = 24;
                            continue;
                        }
                        43808 ... 43814 => {
                            __current_state = 24;
                            continue;
                        }
                        43816 ... 43822 => {
                            __current_state = 24;
                            continue;
                        }
                        43824 ... 43866 => {
                            __current_state = 24;
                            continue;
                        }
                        43868 ... 43877 => {
                            __current_state = 24;
                            continue;
                        }
                        43888 ... 44010 => {
                            __current_state = 24;
                            continue;
                        }
                        44012 ... 44013 => {
                            __current_state = 24;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_state = 24;
                            continue;
                        }
                        44032 ... 55203 => {
                            __current_state = 24;
                            continue;
                        }
                        55216 ... 55238 => {
                            __current_state = 24;
                            continue;
                        }
                        55243 ... 55291 => {
                            __current_state = 24;
                            continue;
                        }
                        63744 ... 64109 => {
                            __current_state = 24;
                            continue;
                        }
                        64112 ... 64217 => {
                            __current_state = 24;
                            continue;
                        }
                        64256 ... 64262 => {
                            __current_state = 24;
                            continue;
                        }
                        64275 ... 64279 => {
                            __current_state = 24;
                            continue;
                        }
                        64285 ... 64296 => {
                            __current_state = 24;
                            continue;
                        }
                        64298 ... 64310 => {
                            __current_state = 24;
                            continue;
                        }
                        64312 ... 64316 => {
                            __current_state = 24;
                            continue;
                        }
                        64318 => /* '\u{fb3e}' */ {
                            __current_state = 24;
                            continue;
                        }
                        64320 ... 64321 => {
                            __current_state = 24;
                            continue;
                        }
                        64323 ... 64324 => {
                            __current_state = 24;
                            continue;
                        }
                        64326 ... 64433 => {
                            __current_state = 24;
                            continue;
                        }
                        64467 ... 64829 => {
                            __current_state = 24;
                            continue;
                        }
                        64848 ... 64911 => {
                            __current_state = 24;
                            continue;
                        }
                        64914 ... 64967 => {
                            __current_state = 24;
                            continue;
                        }
                        65008 ... 65019 => {
                            __current_state = 24;
                            continue;
                        }
                        65024 ... 65039 => {
                            __current_state = 24;
                            continue;
                        }
                        65056 ... 65071 => {
                            __current_state = 24;
                            continue;
                        }
                        65075 ... 65076 => {
                            __current_state = 24;
                            continue;
                        }
                        65101 ... 65103 => {
                            __current_state = 24;
                            continue;
                        }
                        65136 ... 65140 => {
                            __current_state = 24;
                            continue;
                        }
                        65142 ... 65276 => {
                            __current_state = 24;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_state = 24;
                            continue;
                        }
                        65313 ... 65338 => {
                            __current_state = 24;
                            continue;
                        }
                        65343 => /* '\u{ff3f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        65345 ... 65370 => {
                            __current_state = 24;
                            continue;
                        }
                        65382 ... 65470 => {
                            __current_state = 24;
                            continue;
                        }
                        65474 ... 65479 => {
                            __current_state = 24;
                            continue;
                        }
                        65482 ... 65487 => {
                            __current_state = 24;
                            continue;
                        }
                        65490 ... 65495 => {
                            __current_state = 24;
                            continue;
                        }
                        65498 ... 65500 => {
                            __current_state = 24;
                            continue;
                        }
                        65536 ... 65547 => {
                            __current_state = 24;
                            continue;
                        }
                        65549 ... 65574 => {
                            __current_state = 24;
                            continue;
                        }
                        65576 ... 65594 => {
                            __current_state = 24;
                            continue;
                        }
                        65596 ... 65597 => {
                            __current_state = 24;
                            continue;
                        }
                        65599 ... 65613 => {
                            __current_state = 24;
                            continue;
                        }
                        65616 ... 65629 => {
                            __current_state = 24;
                            continue;
                        }
                        65664 ... 65786 => {
                            __current_state = 24;
                            continue;
                        }
                        65856 ... 65908 => {
                            __current_state = 24;
                            continue;
                        }
                        66045 => /* '\u{101fd}' */ {
                            __current_state = 24;
                            continue;
                        }
                        66176 ... 66204 => {
                            __current_state = 24;
                            continue;
                        }
                        66208 ... 66256 => {
                            __current_state = 24;
                            continue;
                        }
                        66272 => /* '\u{102e0}' */ {
                            __current_state = 24;
                            continue;
                        }
                        66304 ... 66335 => {
                            __current_state = 24;
                            continue;
                        }
                        66352 ... 66378 => {
                            __current_state = 24;
                            continue;
                        }
                        66384 ... 66426 => {
                            __current_state = 24;
                            continue;
                        }
                        66432 ... 66461 => {
                            __current_state = 24;
                            continue;
                        }
                        66464 ... 66499 => {
                            __current_state = 24;
                            continue;
                        }
                        66504 ... 66511 => {
                            __current_state = 24;
                            continue;
                        }
                        66513 ... 66517 => {
                            __current_state = 24;
                            continue;
                        }
                        66560 ... 66717 => {
                            __current_state = 24;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_state = 24;
                            continue;
                        }
                        66816 ... 66855 => {
                            __current_state = 24;
                            continue;
                        }
                        66864 ... 66915 => {
                            __current_state = 24;
                            continue;
                        }
                        67072 ... 67382 => {
                            __current_state = 24;
                            continue;
                        }
                        67392 ... 67413 => {
                            __current_state = 24;
                            continue;
                        }
                        67424 ... 67431 => {
                            __current_state = 24;
                            continue;
                        }
                        67584 ... 67589 => {
                            __current_state = 24;
                            continue;
                        }
                        67592 => /* '\u{10808}' */ {
                            __current_state = 24;
                            continue;
                        }
                        67594 ... 67637 => {
                            __current_state = 24;
                            continue;
                        }
                        67639 ... 67640 => {
                            __current_state = 24;
                            continue;
                        }
                        67644 => /* '\u{1083c}' */ {
                            __current_state = 24;
                            continue;
                        }
                        67647 ... 67669 => {
                            __current_state = 24;
                            continue;
                        }
                        67680 ... 67702 => {
                            __current_state = 24;
                            continue;
                        }
                        67712 ... 67742 => {
                            __current_state = 24;
                            continue;
                        }
                        67808 ... 67826 => {
                            __current_state = 24;
                            continue;
                        }
                        67828 ... 67829 => {
                            __current_state = 24;
                            continue;
                        }
                        67840 ... 67861 => {
                            __current_state = 24;
                            continue;
                        }
                        67872 ... 67897 => {
                            __current_state = 24;
                            continue;
                        }
                        67968 ... 68023 => {
                            __current_state = 24;
                            continue;
                        }
                        68030 ... 68031 => {
                            __current_state = 24;
                            continue;
                        }
                        68096 ... 68099 => {
                            __current_state = 24;
                            continue;
                        }
                        68101 ... 68102 => {
                            __current_state = 24;
                            continue;
                        }
                        68108 ... 68115 => {
                            __current_state = 24;
                            continue;
                        }
                        68117 ... 68119 => {
                            __current_state = 24;
                            continue;
                        }
                        68121 ... 68147 => {
                            __current_state = 24;
                            continue;
                        }
                        68152 ... 68154 => {
                            __current_state = 24;
                            continue;
                        }
                        68159 => /* '\u{10a3f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        68192 ... 68220 => {
                            __current_state = 24;
                            continue;
                        }
                        68224 ... 68252 => {
                            __current_state = 24;
                            continue;
                        }
                        68288 ... 68295 => {
                            __current_state = 24;
                            continue;
                        }
                        68297 ... 68326 => {
                            __current_state = 24;
                            continue;
                        }
                        68352 ... 68405 => {
                            __current_state = 24;
                            continue;
                        }
                        68416 ... 68437 => {
                            __current_state = 24;
                            continue;
                        }
                        68448 ... 68466 => {
                            __current_state = 24;
                            continue;
                        }
                        68480 ... 68497 => {
                            __current_state = 24;
                            continue;
                        }
                        68608 ... 68680 => {
                            __current_state = 24;
                            continue;
                        }
                        68736 ... 68786 => {
                            __current_state = 24;
                            continue;
                        }
                        68800 ... 68850 => {
                            __current_state = 24;
                            continue;
                        }
                        69632 ... 69702 => {
                            __current_state = 24;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_state = 24;
                            continue;
                        }
                        69759 ... 69818 => {
                            __current_state = 24;
                            continue;
                        }
                        69840 ... 69864 => {
                            __current_state = 24;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_state = 24;
                            continue;
                        }
                        69888 ... 69940 => {
                            __current_state = 24;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_state = 24;
                            continue;
                        }
                        69968 ... 70003 => {
                            __current_state = 24;
                            continue;
                        }
                        70006 => /* '\u{11176}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70016 ... 70084 => {
                            __current_state = 24;
                            continue;
                        }
                        70090 ... 70092 => {
                            __current_state = 24;
                            continue;
                        }
                        70096 ... 70106 => {
                            __current_state = 24;
                            continue;
                        }
                        70108 => /* '\u{111dc}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70144 ... 70161 => {
                            __current_state = 24;
                            continue;
                        }
                        70163 ... 70199 => {
                            __current_state = 24;
                            continue;
                        }
                        70272 ... 70278 => {
                            __current_state = 24;
                            continue;
                        }
                        70280 => /* '\u{11288}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70282 ... 70285 => {
                            __current_state = 24;
                            continue;
                        }
                        70287 ... 70301 => {
                            __current_state = 24;
                            continue;
                        }
                        70303 ... 70312 => {
                            __current_state = 24;
                            continue;
                        }
                        70320 ... 70378 => {
                            __current_state = 24;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_state = 24;
                            continue;
                        }
                        70400 ... 70403 => {
                            __current_state = 24;
                            continue;
                        }
                        70405 ... 70412 => {
                            __current_state = 24;
                            continue;
                        }
                        70415 ... 70416 => {
                            __current_state = 24;
                            continue;
                        }
                        70419 ... 70440 => {
                            __current_state = 24;
                            continue;
                        }
                        70442 ... 70448 => {
                            __current_state = 24;
                            continue;
                        }
                        70450 ... 70451 => {
                            __current_state = 24;
                            continue;
                        }
                        70453 ... 70457 => {
                            __current_state = 24;
                            continue;
                        }
                        70460 ... 70468 => {
                            __current_state = 24;
                            continue;
                        }
                        70471 ... 70472 => {
                            __current_state = 24;
                            continue;
                        }
                        70475 ... 70477 => {
                            __current_state = 24;
                            continue;
                        }
                        70480 => /* '\u{11350}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70487 => /* '\u{11357}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70493 ... 70499 => {
                            __current_state = 24;
                            continue;
                        }
                        70502 ... 70508 => {
                            __current_state = 24;
                            continue;
                        }
                        70512 ... 70516 => {
                            __current_state = 24;
                            continue;
                        }
                        70784 ... 70853 => {
                            __current_state = 24;
                            continue;
                        }
                        70855 => /* '\u{114c7}' */ {
                            __current_state = 24;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_state = 24;
                            continue;
                        }
                        71040 ... 71093 => {
                            __current_state = 24;
                            continue;
                        }
                        71096 ... 71104 => {
                            __current_state = 24;
                            continue;
                        }
                        71128 ... 71133 => {
                            __current_state = 24;
                            continue;
                        }
                        71168 ... 71232 => {
                            __current_state = 24;
                            continue;
                        }
                        71236 => /* '\u{11644}' */ {
                            __current_state = 24;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_state = 24;
                            continue;
                        }
                        71296 ... 71351 => {
                            __current_state = 24;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_state = 24;
                            continue;
                        }
                        71424 ... 71449 => {
                            __current_state = 24;
                            continue;
                        }
                        71453 ... 71467 => {
                            __current_state = 24;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_state = 24;
                            continue;
                        }
                        71840 ... 71913 => {
                            __current_state = 24;
                            continue;
                        }
                        71935 => /* '\u{118ff}' */ {
                            __current_state = 24;
                            continue;
                        }
                        72384 ... 72440 => {
                            __current_state = 24;
                            continue;
                        }
                        73728 ... 74649 => {
                            __current_state = 24;
                            continue;
                        }
                        74752 ... 74862 => {
                            __current_state = 24;
                            continue;
                        }
                        74880 ... 75075 => {
                            __current_state = 24;
                            continue;
                        }
                        77824 ... 78894 => {
                            __current_state = 24;
                            continue;
                        }
                        82944 ... 83526 => {
                            __current_state = 24;
                            continue;
                        }
                        92160 ... 92728 => {
                            __current_state = 24;
                            continue;
                        }
                        92736 ... 92766 => {
                            __current_state = 24;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_state = 24;
                            continue;
                        }
                        92880 ... 92909 => {
                            __current_state = 24;
                            continue;
                        }
                        92912 ... 92916 => {
                            __current_state = 24;
                            continue;
                        }
                        92928 ... 92982 => {
                            __current_state = 24;
                            continue;
                        }
                        92992 ... 92995 => {
                            __current_state = 24;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_state = 24;
                            continue;
                        }
                        93027 ... 93047 => {
                            __current_state = 24;
                            continue;
                        }
                        93053 ... 93071 => {
                            __current_state = 24;
                            continue;
                        }
                        93952 ... 94020 => {
                            __current_state = 24;
                            continue;
                        }
                        94032 ... 94078 => {
                            __current_state = 24;
                            continue;
                        }
                        94095 ... 94111 => {
                            __current_state = 24;
                            continue;
                        }
                        110592 ... 110593 => {
                            __current_state = 24;
                            continue;
                        }
                        113664 ... 113770 => {
                            __current_state = 24;
                            continue;
                        }
                        113776 ... 113788 => {
                            __current_state = 24;
                            continue;
                        }
                        113792 ... 113800 => {
                            __current_state = 24;
                            continue;
                        }
                        113808 ... 113817 => {
                            __current_state = 24;
                            continue;
                        }
                        113821 ... 113822 => {
                            __current_state = 24;
                            continue;
                        }
                        119141 ... 119145 => {
                            __current_state = 24;
                            continue;
                        }
                        119149 ... 119154 => {
                            __current_state = 24;
                            continue;
                        }
                        119163 ... 119170 => {
                            __current_state = 24;
                            continue;
                        }
                        119173 ... 119179 => {
                            __current_state = 24;
                            continue;
                        }
                        119210 ... 119213 => {
                            __current_state = 24;
                            continue;
                        }
                        119362 ... 119364 => {
                            __current_state = 24;
                            continue;
                        }
                        119808 ... 119892 => {
                            __current_state = 24;
                            continue;
                        }
                        119894 ... 119964 => {
                            __current_state = 24;
                            continue;
                        }
                        119966 ... 119967 => {
                            __current_state = 24;
                            continue;
                        }
                        119970 => /* '\u{1d4a2}' */ {
                            __current_state = 24;
                            continue;
                        }
                        119973 ... 119974 => {
                            __current_state = 24;
                            continue;
                        }
                        119977 ... 119980 => {
                            __current_state = 24;
                            continue;
                        }
                        119982 ... 119993 => {
                            __current_state = 24;
                            continue;
                        }
                        119995 => /* '\u{1d4bb}' */ {
                            __current_state = 24;
                            continue;
                        }
                        119997 ... 120003 => {
                            __current_state = 24;
                            continue;
                        }
                        120005 ... 120069 => {
                            __current_state = 24;
                            continue;
                        }
                        120071 ... 120074 => {
                            __current_state = 24;
                            continue;
                        }
                        120077 ... 120084 => {
                            __current_state = 24;
                            continue;
                        }
                        120086 ... 120092 => {
                            __current_state = 24;
                            continue;
                        }
                        120094 ... 120121 => {
                            __current_state = 24;
                            continue;
                        }
                        120123 ... 120126 => {
                            __current_state = 24;
                            continue;
                        }
                        120128 ... 120132 => {
                            __current_state = 24;
                            continue;
                        }
                        120134 => /* '\u{1d546}' */ {
                            __current_state = 24;
                            continue;
                        }
                        120138 ... 120144 => {
                            __current_state = 24;
                            continue;
                        }
                        120146 ... 120485 => {
                            __current_state = 24;
                            continue;
                        }
                        120488 ... 120512 => {
                            __current_state = 24;
                            continue;
                        }
                        120514 ... 120538 => {
                            __current_state = 24;
                            continue;
                        }
                        120540 ... 120570 => {
                            __current_state = 24;
                            continue;
                        }
                        120572 ... 120596 => {
                            __current_state = 24;
                            continue;
                        }
                        120598 ... 120628 => {
                            __current_state = 24;
                            continue;
                        }
                        120630 ... 120654 => {
                            __current_state = 24;
                            continue;
                        }
                        120656 ... 120686 => {
                            __current_state = 24;
                            continue;
                        }
                        120688 ... 120712 => {
                            __current_state = 24;
                            continue;
                        }
                        120714 ... 120744 => {
                            __current_state = 24;
                            continue;
                        }
                        120746 ... 120770 => {
                            __current_state = 24;
                            continue;
                        }
                        120772 ... 120779 => {
                            __current_state = 24;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_state = 24;
                            continue;
                        }
                        121344 ... 121398 => {
                            __current_state = 24;
                            continue;
                        }
                        121403 ... 121452 => {
                            __current_state = 24;
                            continue;
                        }
                        121461 => /* '\u{1da75}' */ {
                            __current_state = 24;
                            continue;
                        }
                        121476 => /* '\u{1da84}' */ {
                            __current_state = 24;
                            continue;
                        }
                        121499 ... 121503 => {
                            __current_state = 24;
                            continue;
                        }
                        121505 ... 121519 => {
                            __current_state = 24;
                            continue;
                        }
                        124928 ... 125124 => {
                            __current_state = 24;
                            continue;
                        }
                        125136 ... 125142 => {
                            __current_state = 24;
                            continue;
                        }
                        126464 ... 126467 => {
                            __current_state = 24;
                            continue;
                        }
                        126469 ... 126495 => {
                            __current_state = 24;
                            continue;
                        }
                        126497 ... 126498 => {
                            __current_state = 24;
                            continue;
                        }
                        126500 => /* '\u{1ee24}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126503 => /* '\u{1ee27}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126505 ... 126514 => {
                            __current_state = 24;
                            continue;
                        }
                        126516 ... 126519 => {
                            __current_state = 24;
                            continue;
                        }
                        126521 => /* '\u{1ee39}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126523 => /* '\u{1ee3b}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126530 => /* '\u{1ee42}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126535 => /* '\u{1ee47}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126537 => /* '\u{1ee49}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126539 => /* '\u{1ee4b}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126541 ... 126543 => {
                            __current_state = 24;
                            continue;
                        }
                        126545 ... 126546 => {
                            __current_state = 24;
                            continue;
                        }
                        126548 => /* '\u{1ee54}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126551 => /* '\u{1ee57}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126553 => /* '\u{1ee59}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126555 => /* '\u{1ee5b}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126557 => /* '\u{1ee5d}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126559 => /* '\u{1ee5f}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126561 ... 126562 => {
                            __current_state = 24;
                            continue;
                        }
                        126564 => /* '\u{1ee64}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126567 ... 126570 => {
                            __current_state = 24;
                            continue;
                        }
                        126572 ... 126578 => {
                            __current_state = 24;
                            continue;
                        }
                        126580 ... 126583 => {
                            __current_state = 24;
                            continue;
                        }
                        126585 ... 126588 => {
                            __current_state = 24;
                            continue;
                        }
                        126590 => /* '\u{1ee7e}' */ {
                            __current_state = 24;
                            continue;
                        }
                        126592 ... 126601 => {
                            __current_state = 24;
                            continue;
                        }
                        126603 ... 126619 => {
                            __current_state = 24;
                            continue;
                        }
                        126625 ... 126627 => {
                            __current_state = 24;
                            continue;
                        }
                        126629 ... 126633 => {
                            __current_state = 24;
                            continue;
                        }
                        126635 ... 126651 => {
                            __current_state = 24;
                            continue;
                        }
                        127280 ... 127305 => {
                            __current_state = 24;
                            continue;
                        }
                        127312 ... 127337 => {
                            __current_state = 24;
                            continue;
                        }
                        127344 ... 127369 => {
                            __current_state = 24;
                            continue;
                        }
                        131072 ... 173782 => {
                            __current_state = 24;
                            continue;
                        }
                        173824 ... 177972 => {
                            __current_state = 24;
                            continue;
                        }
                        177984 ... 178205 => {
                            __current_state = 24;
                            continue;
                        }
                        178208 ... 183969 => {
                            __current_state = 24;
                            continue;
                        }
                        194560 ... 195101 => {
                            __current_state = 24;
                            continue;
                        }
                        917760 ... 917999 => {
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        _ => {
                            __current_match = Some((9, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        62 => /* '>' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((44, __index + __ch.len_utf8()));
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 29;
                            continue;
                        }
                        68 => /* 'D' */ {
                            __current_state = 30;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_state = 31;
                            continue;
                        }
                        77 => /* 'M' */ {
                            __current_state = 32;
                            continue;
                        }
                        87 => /* 'W' */ {
                            __current_state = 33;
                            continue;
                        }
                        89 => /* 'Y' */ {
                            __current_state = 34;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 29;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_state = 30;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 31;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 32;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_state = 33;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        80 => /* 'P' */ {
                            __current_state = 35;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_state = 36;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 35;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 36;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 37;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_state = 38;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 37;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 38;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 39;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 39;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 40;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_state = 41;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 40;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 41;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 42;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_state = 43;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 42;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 43;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 44;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_state = 45;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 44;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 45;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 46;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 46;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        67 => /* 'C' */ {
                            __current_state = 47;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_state = 47;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        85 => /* 'U' */ {
                            __current_state = 48;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 48;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 49;
                            continue;
                        }
                        69 => /* 'E' */ {
                            __current_state = 50;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_state = 51;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 49;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 50;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 51;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 52;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_state = 53;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_state = 54;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 52;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 53;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 54;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 55;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 55;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 56;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_state = 58;
                            continue;
                        }
                        48 ... 57 => {
                            __current_state = 58;
                            continue;
                        }
                        64 ... 90 => {
                            __current_state = 58;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_state = 58;
                            continue;
                        }
                        97 ... 122 => {
                            __current_state = 58;
                            continue;
                        }
                        170 => /* '\u{aa}' */ {
                            __current_state = 58;
                            continue;
                        }
                        181 => /* '\u{b5}' */ {
                            __current_state = 58;
                            continue;
                        }
                        186 => /* '\u{ba}' */ {
                            __current_state = 58;
                            continue;
                        }
                        192 ... 214 => {
                            __current_state = 58;
                            continue;
                        }
                        216 ... 246 => {
                            __current_state = 58;
                            continue;
                        }
                        248 ... 705 => {
                            __current_state = 58;
                            continue;
                        }
                        710 ... 721 => {
                            __current_state = 58;
                            continue;
                        }
                        736 ... 740 => {
                            __current_state = 58;
                            continue;
                        }
                        748 => /* '\u{2ec}' */ {
                            __current_state = 58;
                            continue;
                        }
                        750 => /* '\u{2ee}' */ {
                            __current_state = 58;
                            continue;
                        }
                        768 ... 884 => {
                            __current_state = 58;
                            continue;
                        }
                        886 ... 887 => {
                            __current_state = 58;
                            continue;
                        }
                        890 ... 893 => {
                            __current_state = 58;
                            continue;
                        }
                        895 => /* '\u{37f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        902 => /* '\u{386}' */ {
                            __current_state = 58;
                            continue;
                        }
                        904 ... 906 => {
                            __current_state = 58;
                            continue;
                        }
                        908 => /* '\u{38c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        910 ... 929 => {
                            __current_state = 58;
                            continue;
                        }
                        931 ... 1013 => {
                            __current_state = 58;
                            continue;
                        }
                        1015 ... 1153 => {
                            __current_state = 58;
                            continue;
                        }
                        1155 ... 1327 => {
                            __current_state = 58;
                            continue;
                        }
                        1329 ... 1366 => {
                            __current_state = 58;
                            continue;
                        }
                        1369 => /* '\u{559}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1377 ... 1415 => {
                            __current_state = 58;
                            continue;
                        }
                        1425 ... 1469 => {
                            __current_state = 58;
                            continue;
                        }
                        1471 => /* '\u{5bf}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1473 ... 1474 => {
                            __current_state = 58;
                            continue;
                        }
                        1476 ... 1477 => {
                            __current_state = 58;
                            continue;
                        }
                        1479 => /* '\u{5c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1488 ... 1514 => {
                            __current_state = 58;
                            continue;
                        }
                        1520 ... 1522 => {
                            __current_state = 58;
                            continue;
                        }
                        1552 ... 1562 => {
                            __current_state = 58;
                            continue;
                        }
                        1568 ... 1641 => {
                            __current_state = 58;
                            continue;
                        }
                        1646 ... 1747 => {
                            __current_state = 58;
                            continue;
                        }
                        1749 ... 1756 => {
                            __current_state = 58;
                            continue;
                        }
                        1759 ... 1768 => {
                            __current_state = 58;
                            continue;
                        }
                        1770 ... 1788 => {
                            __current_state = 58;
                            continue;
                        }
                        1791 => /* '\u{6ff}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1808 ... 1866 => {
                            __current_state = 58;
                            continue;
                        }
                        1869 ... 1969 => {
                            __current_state = 58;
                            continue;
                        }
                        1984 ... 2037 => {
                            __current_state = 58;
                            continue;
                        }
                        2042 => /* '\u{7fa}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2048 ... 2093 => {
                            __current_state = 58;
                            continue;
                        }
                        2112 ... 2139 => {
                            __current_state = 58;
                            continue;
                        }
                        2208 ... 2228 => {
                            __current_state = 58;
                            continue;
                        }
                        2275 ... 2403 => {
                            __current_state = 58;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_state = 58;
                            continue;
                        }
                        2417 ... 2435 => {
                            __current_state = 58;
                            continue;
                        }
                        2437 ... 2444 => {
                            __current_state = 58;
                            continue;
                        }
                        2447 ... 2448 => {
                            __current_state = 58;
                            continue;
                        }
                        2451 ... 2472 => {
                            __current_state = 58;
                            continue;
                        }
                        2474 ... 2480 => {
                            __current_state = 58;
                            continue;
                        }
                        2482 => /* '\u{9b2}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2486 ... 2489 => {
                            __current_state = 58;
                            continue;
                        }
                        2492 ... 2500 => {
                            __current_state = 58;
                            continue;
                        }
                        2503 ... 2504 => {
                            __current_state = 58;
                            continue;
                        }
                        2507 ... 2510 => {
                            __current_state = 58;
                            continue;
                        }
                        2519 => /* '\u{9d7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2524 ... 2525 => {
                            __current_state = 58;
                            continue;
                        }
                        2527 ... 2531 => {
                            __current_state = 58;
                            continue;
                        }
                        2534 ... 2545 => {
                            __current_state = 58;
                            continue;
                        }
                        2561 ... 2563 => {
                            __current_state = 58;
                            continue;
                        }
                        2565 ... 2570 => {
                            __current_state = 58;
                            continue;
                        }
                        2575 ... 2576 => {
                            __current_state = 58;
                            continue;
                        }
                        2579 ... 2600 => {
                            __current_state = 58;
                            continue;
                        }
                        2602 ... 2608 => {
                            __current_state = 58;
                            continue;
                        }
                        2610 ... 2611 => {
                            __current_state = 58;
                            continue;
                        }
                        2613 ... 2614 => {
                            __current_state = 58;
                            continue;
                        }
                        2616 ... 2617 => {
                            __current_state = 58;
                            continue;
                        }
                        2620 => /* '\u{a3c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2622 ... 2626 => {
                            __current_state = 58;
                            continue;
                        }
                        2631 ... 2632 => {
                            __current_state = 58;
                            continue;
                        }
                        2635 ... 2637 => {
                            __current_state = 58;
                            continue;
                        }
                        2641 => /* '\u{a51}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2649 ... 2652 => {
                            __current_state = 58;
                            continue;
                        }
                        2654 => /* '\u{a5e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2662 ... 2677 => {
                            __current_state = 58;
                            continue;
                        }
                        2689 ... 2691 => {
                            __current_state = 58;
                            continue;
                        }
                        2693 ... 2701 => {
                            __current_state = 58;
                            continue;
                        }
                        2703 ... 2705 => {
                            __current_state = 58;
                            continue;
                        }
                        2707 ... 2728 => {
                            __current_state = 58;
                            continue;
                        }
                        2730 ... 2736 => {
                            __current_state = 58;
                            continue;
                        }
                        2738 ... 2739 => {
                            __current_state = 58;
                            continue;
                        }
                        2741 ... 2745 => {
                            __current_state = 58;
                            continue;
                        }
                        2748 ... 2757 => {
                            __current_state = 58;
                            continue;
                        }
                        2759 ... 2761 => {
                            __current_state = 58;
                            continue;
                        }
                        2763 ... 2765 => {
                            __current_state = 58;
                            continue;
                        }
                        2768 => /* '\u{ad0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2784 ... 2787 => {
                            __current_state = 58;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_state = 58;
                            continue;
                        }
                        2809 => /* '\u{af9}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2817 ... 2819 => {
                            __current_state = 58;
                            continue;
                        }
                        2821 ... 2828 => {
                            __current_state = 58;
                            continue;
                        }
                        2831 ... 2832 => {
                            __current_state = 58;
                            continue;
                        }
                        2835 ... 2856 => {
                            __current_state = 58;
                            continue;
                        }
                        2858 ... 2864 => {
                            __current_state = 58;
                            continue;
                        }
                        2866 ... 2867 => {
                            __current_state = 58;
                            continue;
                        }
                        2869 ... 2873 => {
                            __current_state = 58;
                            continue;
                        }
                        2876 ... 2884 => {
                            __current_state = 58;
                            continue;
                        }
                        2887 ... 2888 => {
                            __current_state = 58;
                            continue;
                        }
                        2891 ... 2893 => {
                            __current_state = 58;
                            continue;
                        }
                        2902 ... 2903 => {
                            __current_state = 58;
                            continue;
                        }
                        2908 ... 2909 => {
                            __current_state = 58;
                            continue;
                        }
                        2911 ... 2915 => {
                            __current_state = 58;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_state = 58;
                            continue;
                        }
                        2929 => /* '\u{b71}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2946 ... 2947 => {
                            __current_state = 58;
                            continue;
                        }
                        2949 ... 2954 => {
                            __current_state = 58;
                            continue;
                        }
                        2958 ... 2960 => {
                            __current_state = 58;
                            continue;
                        }
                        2962 ... 2965 => {
                            __current_state = 58;
                            continue;
                        }
                        2969 ... 2970 => {
                            __current_state = 58;
                            continue;
                        }
                        2972 => /* '\u{b9c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2974 ... 2975 => {
                            __current_state = 58;
                            continue;
                        }
                        2979 ... 2980 => {
                            __current_state = 58;
                            continue;
                        }
                        2984 ... 2986 => {
                            __current_state = 58;
                            continue;
                        }
                        2990 ... 3001 => {
                            __current_state = 58;
                            continue;
                        }
                        3006 ... 3010 => {
                            __current_state = 58;
                            continue;
                        }
                        3014 ... 3016 => {
                            __current_state = 58;
                            continue;
                        }
                        3018 ... 3021 => {
                            __current_state = 58;
                            continue;
                        }
                        3024 => /* '\u{bd0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3031 => /* '\u{bd7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_state = 58;
                            continue;
                        }
                        3072 ... 3075 => {
                            __current_state = 58;
                            continue;
                        }
                        3077 ... 3084 => {
                            __current_state = 58;
                            continue;
                        }
                        3086 ... 3088 => {
                            __current_state = 58;
                            continue;
                        }
                        3090 ... 3112 => {
                            __current_state = 58;
                            continue;
                        }
                        3114 ... 3129 => {
                            __current_state = 58;
                            continue;
                        }
                        3133 ... 3140 => {
                            __current_state = 58;
                            continue;
                        }
                        3142 ... 3144 => {
                            __current_state = 58;
                            continue;
                        }
                        3146 ... 3149 => {
                            __current_state = 58;
                            continue;
                        }
                        3157 ... 3158 => {
                            __current_state = 58;
                            continue;
                        }
                        3160 ... 3162 => {
                            __current_state = 58;
                            continue;
                        }
                        3168 ... 3171 => {
                            __current_state = 58;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_state = 58;
                            continue;
                        }
                        3201 ... 3203 => {
                            __current_state = 58;
                            continue;
                        }
                        3205 ... 3212 => {
                            __current_state = 58;
                            continue;
                        }
                        3214 ... 3216 => {
                            __current_state = 58;
                            continue;
                        }
                        3218 ... 3240 => {
                            __current_state = 58;
                            continue;
                        }
                        3242 ... 3251 => {
                            __current_state = 58;
                            continue;
                        }
                        3253 ... 3257 => {
                            __current_state = 58;
                            continue;
                        }
                        3260 ... 3268 => {
                            __current_state = 58;
                            continue;
                        }
                        3270 ... 3272 => {
                            __current_state = 58;
                            continue;
                        }
                        3274 ... 3277 => {
                            __current_state = 58;
                            continue;
                        }
                        3285 ... 3286 => {
                            __current_state = 58;
                            continue;
                        }
                        3294 => /* '\u{cde}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3296 ... 3299 => {
                            __current_state = 58;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_state = 58;
                            continue;
                        }
                        3313 ... 3314 => {
                            __current_state = 58;
                            continue;
                        }
                        3329 ... 3331 => {
                            __current_state = 58;
                            continue;
                        }
                        3333 ... 3340 => {
                            __current_state = 58;
                            continue;
                        }
                        3342 ... 3344 => {
                            __current_state = 58;
                            continue;
                        }
                        3346 ... 3386 => {
                            __current_state = 58;
                            continue;
                        }
                        3389 ... 3396 => {
                            __current_state = 58;
                            continue;
                        }
                        3398 ... 3400 => {
                            __current_state = 58;
                            continue;
                        }
                        3402 ... 3406 => {
                            __current_state = 58;
                            continue;
                        }
                        3415 => /* '\u{d57}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3423 ... 3427 => {
                            __current_state = 58;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_state = 58;
                            continue;
                        }
                        3450 ... 3455 => {
                            __current_state = 58;
                            continue;
                        }
                        3458 ... 3459 => {
                            __current_state = 58;
                            continue;
                        }
                        3461 ... 3478 => {
                            __current_state = 58;
                            continue;
                        }
                        3482 ... 3505 => {
                            __current_state = 58;
                            continue;
                        }
                        3507 ... 3515 => {
                            __current_state = 58;
                            continue;
                        }
                        3517 => /* '\u{dbd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3520 ... 3526 => {
                            __current_state = 58;
                            continue;
                        }
                        3530 => /* '\u{dca}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3535 ... 3540 => {
                            __current_state = 58;
                            continue;
                        }
                        3542 => /* '\u{dd6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3544 ... 3551 => {
                            __current_state = 58;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_state = 58;
                            continue;
                        }
                        3570 ... 3571 => {
                            __current_state = 58;
                            continue;
                        }
                        3585 ... 3642 => {
                            __current_state = 58;
                            continue;
                        }
                        3648 ... 3662 => {
                            __current_state = 58;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_state = 58;
                            continue;
                        }
                        3713 ... 3714 => {
                            __current_state = 58;
                            continue;
                        }
                        3716 => /* '\u{e84}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3719 ... 3720 => {
                            __current_state = 58;
                            continue;
                        }
                        3722 => /* '\u{e8a}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3725 => /* '\u{e8d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3732 ... 3735 => {
                            __current_state = 58;
                            continue;
                        }
                        3737 ... 3743 => {
                            __current_state = 58;
                            continue;
                        }
                        3745 ... 3747 => {
                            __current_state = 58;
                            continue;
                        }
                        3749 => /* '\u{ea5}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3751 => /* '\u{ea7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3754 ... 3755 => {
                            __current_state = 58;
                            continue;
                        }
                        3757 ... 3769 => {
                            __current_state = 58;
                            continue;
                        }
                        3771 ... 3773 => {
                            __current_state = 58;
                            continue;
                        }
                        3776 ... 3780 => {
                            __current_state = 58;
                            continue;
                        }
                        3782 => /* '\u{ec6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3784 ... 3789 => {
                            __current_state = 58;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_state = 58;
                            continue;
                        }
                        3804 ... 3807 => {
                            __current_state = 58;
                            continue;
                        }
                        3840 => /* '\u{f00}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3864 ... 3865 => {
                            __current_state = 58;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_state = 58;
                            continue;
                        }
                        3893 => /* '\u{f35}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3895 => /* '\u{f37}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3897 => /* '\u{f39}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3902 ... 3911 => {
                            __current_state = 58;
                            continue;
                        }
                        3913 ... 3948 => {
                            __current_state = 58;
                            continue;
                        }
                        3953 ... 3972 => {
                            __current_state = 58;
                            continue;
                        }
                        3974 ... 3991 => {
                            __current_state = 58;
                            continue;
                        }
                        3993 ... 4028 => {
                            __current_state = 58;
                            continue;
                        }
                        4038 => /* '\u{fc6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4096 ... 4169 => {
                            __current_state = 58;
                            continue;
                        }
                        4176 ... 4253 => {
                            __current_state = 58;
                            continue;
                        }
                        4256 ... 4293 => {
                            __current_state = 58;
                            continue;
                        }
                        4295 => /* '\u{10c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4301 => /* '\u{10cd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4304 ... 4346 => {
                            __current_state = 58;
                            continue;
                        }
                        4348 ... 4680 => {
                            __current_state = 58;
                            continue;
                        }
                        4682 ... 4685 => {
                            __current_state = 58;
                            continue;
                        }
                        4688 ... 4694 => {
                            __current_state = 58;
                            continue;
                        }
                        4696 => /* '\u{1258}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4698 ... 4701 => {
                            __current_state = 58;
                            continue;
                        }
                        4704 ... 4744 => {
                            __current_state = 58;
                            continue;
                        }
                        4746 ... 4749 => {
                            __current_state = 58;
                            continue;
                        }
                        4752 ... 4784 => {
                            __current_state = 58;
                            continue;
                        }
                        4786 ... 4789 => {
                            __current_state = 58;
                            continue;
                        }
                        4792 ... 4798 => {
                            __current_state = 58;
                            continue;
                        }
                        4800 => /* '\u{12c0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4802 ... 4805 => {
                            __current_state = 58;
                            continue;
                        }
                        4808 ... 4822 => {
                            __current_state = 58;
                            continue;
                        }
                        4824 ... 4880 => {
                            __current_state = 58;
                            continue;
                        }
                        4882 ... 4885 => {
                            __current_state = 58;
                            continue;
                        }
                        4888 ... 4954 => {
                            __current_state = 58;
                            continue;
                        }
                        4957 ... 4959 => {
                            __current_state = 58;
                            continue;
                        }
                        4992 ... 5007 => {
                            __current_state = 58;
                            continue;
                        }
                        5024 ... 5109 => {
                            __current_state = 58;
                            continue;
                        }
                        5112 ... 5117 => {
                            __current_state = 58;
                            continue;
                        }
                        5121 ... 5740 => {
                            __current_state = 58;
                            continue;
                        }
                        5743 ... 5759 => {
                            __current_state = 58;
                            continue;
                        }
                        5761 ... 5786 => {
                            __current_state = 58;
                            continue;
                        }
                        5792 ... 5866 => {
                            __current_state = 58;
                            continue;
                        }
                        5870 ... 5880 => {
                            __current_state = 58;
                            continue;
                        }
                        5888 ... 5900 => {
                            __current_state = 58;
                            continue;
                        }
                        5902 ... 5908 => {
                            __current_state = 58;
                            continue;
                        }
                        5920 ... 5940 => {
                            __current_state = 58;
                            continue;
                        }
                        5952 ... 5971 => {
                            __current_state = 58;
                            continue;
                        }
                        5984 ... 5996 => {
                            __current_state = 58;
                            continue;
                        }
                        5998 ... 6000 => {
                            __current_state = 58;
                            continue;
                        }
                        6002 ... 6003 => {
                            __current_state = 58;
                            continue;
                        }
                        6016 ... 6099 => {
                            __current_state = 58;
                            continue;
                        }
                        6103 => /* '\u{17d7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        6108 ... 6109 => {
                            __current_state = 58;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_state = 58;
                            continue;
                        }
                        6155 ... 6157 => {
                            __current_state = 58;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_state = 58;
                            continue;
                        }
                        6176 ... 6263 => {
                            __current_state = 58;
                            continue;
                        }
                        6272 ... 6314 => {
                            __current_state = 58;
                            continue;
                        }
                        6320 ... 6389 => {
                            __current_state = 58;
                            continue;
                        }
                        6400 ... 6430 => {
                            __current_state = 58;
                            continue;
                        }
                        6432 ... 6443 => {
                            __current_state = 58;
                            continue;
                        }
                        6448 ... 6459 => {
                            __current_state = 58;
                            continue;
                        }
                        6470 ... 6509 => {
                            __current_state = 58;
                            continue;
                        }
                        6512 ... 6516 => {
                            __current_state = 58;
                            continue;
                        }
                        6528 ... 6571 => {
                            __current_state = 58;
                            continue;
                        }
                        6576 ... 6601 => {
                            __current_state = 58;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_state = 58;
                            continue;
                        }
                        6656 ... 6683 => {
                            __current_state = 58;
                            continue;
                        }
                        6688 ... 6750 => {
                            __current_state = 58;
                            continue;
                        }
                        6752 ... 6780 => {
                            __current_state = 58;
                            continue;
                        }
                        6783 ... 6793 => {
                            __current_state = 58;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_state = 58;
                            continue;
                        }
                        6823 => /* '\u{1aa7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        6832 ... 6846 => {
                            __current_state = 58;
                            continue;
                        }
                        6912 ... 6987 => {
                            __current_state = 58;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_state = 58;
                            continue;
                        }
                        7019 ... 7027 => {
                            __current_state = 58;
                            continue;
                        }
                        7040 ... 7155 => {
                            __current_state = 58;
                            continue;
                        }
                        7168 ... 7223 => {
                            __current_state = 58;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_state = 58;
                            continue;
                        }
                        7245 ... 7293 => {
                            __current_state = 58;
                            continue;
                        }
                        7376 ... 7378 => {
                            __current_state = 58;
                            continue;
                        }
                        7380 ... 7414 => {
                            __current_state = 58;
                            continue;
                        }
                        7416 ... 7417 => {
                            __current_state = 58;
                            continue;
                        }
                        7424 ... 7669 => {
                            __current_state = 58;
                            continue;
                        }
                        7676 ... 7957 => {
                            __current_state = 58;
                            continue;
                        }
                        7960 ... 7965 => {
                            __current_state = 58;
                            continue;
                        }
                        7968 ... 8005 => {
                            __current_state = 58;
                            continue;
                        }
                        8008 ... 8013 => {
                            __current_state = 58;
                            continue;
                        }
                        8016 ... 8023 => {
                            __current_state = 58;
                            continue;
                        }
                        8025 => /* '\u{1f59}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8027 => /* '\u{1f5b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8029 => /* '\u{1f5d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8031 ... 8061 => {
                            __current_state = 58;
                            continue;
                        }
                        8064 ... 8116 => {
                            __current_state = 58;
                            continue;
                        }
                        8118 ... 8124 => {
                            __current_state = 58;
                            continue;
                        }
                        8126 => /* '\u{1fbe}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8130 ... 8132 => {
                            __current_state = 58;
                            continue;
                        }
                        8134 ... 8140 => {
                            __current_state = 58;
                            continue;
                        }
                        8144 ... 8147 => {
                            __current_state = 58;
                            continue;
                        }
                        8150 ... 8155 => {
                            __current_state = 58;
                            continue;
                        }
                        8160 ... 8172 => {
                            __current_state = 58;
                            continue;
                        }
                        8178 ... 8180 => {
                            __current_state = 58;
                            continue;
                        }
                        8182 ... 8188 => {
                            __current_state = 58;
                            continue;
                        }
                        8204 ... 8205 => {
                            __current_state = 58;
                            continue;
                        }
                        8255 ... 8256 => {
                            __current_state = 58;
                            continue;
                        }
                        8276 => /* '\u{2054}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8305 => /* '\u{2071}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8319 => /* '\u{207f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8336 ... 8348 => {
                            __current_state = 58;
                            continue;
                        }
                        8400 ... 8432 => {
                            __current_state = 58;
                            continue;
                        }
                        8450 => /* '\u{2102}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8455 => /* '\u{2107}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8458 ... 8467 => {
                            __current_state = 58;
                            continue;
                        }
                        8469 => /* '\u{2115}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8473 ... 8477 => {
                            __current_state = 58;
                            continue;
                        }
                        8484 => /* '\u{2124}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8486 => /* '\u{2126}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8488 => /* '\u{2128}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8490 ... 8493 => {
                            __current_state = 58;
                            continue;
                        }
                        8495 ... 8505 => {
                            __current_state = 58;
                            continue;
                        }
                        8508 ... 8511 => {
                            __current_state = 58;
                            continue;
                        }
                        8517 ... 8521 => {
                            __current_state = 58;
                            continue;
                        }
                        8526 => /* '\u{214e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8544 ... 8584 => {
                            __current_state = 58;
                            continue;
                        }
                        9398 ... 9449 => {
                            __current_state = 58;
                            continue;
                        }
                        11264 ... 11310 => {
                            __current_state = 58;
                            continue;
                        }
                        11312 ... 11358 => {
                            __current_state = 58;
                            continue;
                        }
                        11360 ... 11492 => {
                            __current_state = 58;
                            continue;
                        }
                        11499 ... 11507 => {
                            __current_state = 58;
                            continue;
                        }
                        11520 ... 11557 => {
                            __current_state = 58;
                            continue;
                        }
                        11559 => /* '\u{2d27}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11565 => /* '\u{2d2d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11568 ... 11623 => {
                            __current_state = 58;
                            continue;
                        }
                        11631 => /* '\u{2d6f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11647 ... 11670 => {
                            __current_state = 58;
                            continue;
                        }
                        11680 ... 11686 => {
                            __current_state = 58;
                            continue;
                        }
                        11688 ... 11694 => {
                            __current_state = 58;
                            continue;
                        }
                        11696 ... 11702 => {
                            __current_state = 58;
                            continue;
                        }
                        11704 ... 11710 => {
                            __current_state = 58;
                            continue;
                        }
                        11712 ... 11718 => {
                            __current_state = 58;
                            continue;
                        }
                        11720 ... 11726 => {
                            __current_state = 58;
                            continue;
                        }
                        11728 ... 11734 => {
                            __current_state = 58;
                            continue;
                        }
                        11736 ... 11742 => {
                            __current_state = 58;
                            continue;
                        }
                        11744 ... 11775 => {
                            __current_state = 58;
                            continue;
                        }
                        11823 => /* '\u{2e2f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        12293 ... 12295 => {
                            __current_state = 58;
                            continue;
                        }
                        12321 ... 12335 => {
                            __current_state = 58;
                            continue;
                        }
                        12337 ... 12341 => {
                            __current_state = 58;
                            continue;
                        }
                        12344 ... 12348 => {
                            __current_state = 58;
                            continue;
                        }
                        12353 ... 12438 => {
                            __current_state = 58;
                            continue;
                        }
                        12441 ... 12442 => {
                            __current_state = 58;
                            continue;
                        }
                        12445 ... 12447 => {
                            __current_state = 58;
                            continue;
                        }
                        12449 ... 12538 => {
                            __current_state = 58;
                            continue;
                        }
                        12540 ... 12543 => {
                            __current_state = 58;
                            continue;
                        }
                        12549 ... 12589 => {
                            __current_state = 58;
                            continue;
                        }
                        12593 ... 12686 => {
                            __current_state = 58;
                            continue;
                        }
                        12704 ... 12730 => {
                            __current_state = 58;
                            continue;
                        }
                        12784 ... 12799 => {
                            __current_state = 58;
                            continue;
                        }
                        13312 ... 19893 => {
                            __current_state = 58;
                            continue;
                        }
                        19968 ... 40917 => {
                            __current_state = 58;
                            continue;
                        }
                        40960 ... 42124 => {
                            __current_state = 58;
                            continue;
                        }
                        42192 ... 42237 => {
                            __current_state = 58;
                            continue;
                        }
                        42240 ... 42508 => {
                            __current_state = 58;
                            continue;
                        }
                        42512 ... 42539 => {
                            __current_state = 58;
                            continue;
                        }
                        42560 ... 42610 => {
                            __current_state = 58;
                            continue;
                        }
                        42612 ... 42621 => {
                            __current_state = 58;
                            continue;
                        }
                        42623 ... 42737 => {
                            __current_state = 58;
                            continue;
                        }
                        42775 ... 42783 => {
                            __current_state = 58;
                            continue;
                        }
                        42786 ... 42888 => {
                            __current_state = 58;
                            continue;
                        }
                        42891 ... 42925 => {
                            __current_state = 58;
                            continue;
                        }
                        42928 ... 42935 => {
                            __current_state = 58;
                            continue;
                        }
                        42999 ... 43047 => {
                            __current_state = 58;
                            continue;
                        }
                        43072 ... 43123 => {
                            __current_state = 58;
                            continue;
                        }
                        43136 ... 43204 => {
                            __current_state = 58;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_state = 58;
                            continue;
                        }
                        43232 ... 43255 => {
                            __current_state = 58;
                            continue;
                        }
                        43259 => /* '\u{a8fb}' */ {
                            __current_state = 58;
                            continue;
                        }
                        43261 => /* '\u{a8fd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        43264 ... 43309 => {
                            __current_state = 58;
                            continue;
                        }
                        43312 ... 43347 => {
                            __current_state = 58;
                            continue;
                        }
                        43360 ... 43388 => {
                            __current_state = 58;
                            continue;
                        }
                        43392 ... 43456 => {
                            __current_state = 58;
                            continue;
                        }
                        43471 ... 43481 => {
                            __current_state = 58;
                            continue;
                        }
                        43488 ... 43518 => {
                            __current_state = 58;
                            continue;
                        }
                        43520 ... 43574 => {
                            __current_state = 58;
                            continue;
                        }
                        43584 ... 43597 => {
                            __current_state = 58;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_state = 58;
                            continue;
                        }
                        43616 ... 43638 => {
                            __current_state = 58;
                            continue;
                        }
                        43642 ... 43714 => {
                            __current_state = 58;
                            continue;
                        }
                        43739 ... 43741 => {
                            __current_state = 58;
                            continue;
                        }
                        43744 ... 43759 => {
                            __current_state = 58;
                            continue;
                        }
                        43762 ... 43766 => {
                            __current_state = 58;
                            continue;
                        }
                        43777 ... 43782 => {
                            __current_state = 58;
                            continue;
                        }
                        43785 ... 43790 => {
                            __current_state = 58;
                            continue;
                        }
                        43793 ... 43798 => {
                            __current_state = 58;
                            continue;
                        }
                        43808 ... 43814 => {
                            __current_state = 58;
                            continue;
                        }
                        43816 ... 43822 => {
                            __current_state = 58;
                            continue;
                        }
                        43824 ... 43866 => {
                            __current_state = 58;
                            continue;
                        }
                        43868 ... 43877 => {
                            __current_state = 58;
                            continue;
                        }
                        43888 ... 44010 => {
                            __current_state = 58;
                            continue;
                        }
                        44012 ... 44013 => {
                            __current_state = 58;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_state = 58;
                            continue;
                        }
                        44032 ... 55203 => {
                            __current_state = 58;
                            continue;
                        }
                        55216 ... 55238 => {
                            __current_state = 58;
                            continue;
                        }
                        55243 ... 55291 => {
                            __current_state = 58;
                            continue;
                        }
                        63744 ... 64109 => {
                            __current_state = 58;
                            continue;
                        }
                        64112 ... 64217 => {
                            __current_state = 58;
                            continue;
                        }
                        64256 ... 64262 => {
                            __current_state = 58;
                            continue;
                        }
                        64275 ... 64279 => {
                            __current_state = 58;
                            continue;
                        }
                        64285 ... 64296 => {
                            __current_state = 58;
                            continue;
                        }
                        64298 ... 64310 => {
                            __current_state = 58;
                            continue;
                        }
                        64312 ... 64316 => {
                            __current_state = 58;
                            continue;
                        }
                        64318 => /* '\u{fb3e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        64320 ... 64321 => {
                            __current_state = 58;
                            continue;
                        }
                        64323 ... 64324 => {
                            __current_state = 58;
                            continue;
                        }
                        64326 ... 64433 => {
                            __current_state = 58;
                            continue;
                        }
                        64467 ... 64829 => {
                            __current_state = 58;
                            continue;
                        }
                        64848 ... 64911 => {
                            __current_state = 58;
                            continue;
                        }
                        64914 ... 64967 => {
                            __current_state = 58;
                            continue;
                        }
                        65008 ... 65019 => {
                            __current_state = 58;
                            continue;
                        }
                        65024 ... 65039 => {
                            __current_state = 58;
                            continue;
                        }
                        65056 ... 65071 => {
                            __current_state = 58;
                            continue;
                        }
                        65075 ... 65076 => {
                            __current_state = 58;
                            continue;
                        }
                        65101 ... 65103 => {
                            __current_state = 58;
                            continue;
                        }
                        65136 ... 65140 => {
                            __current_state = 58;
                            continue;
                        }
                        65142 ... 65276 => {
                            __current_state = 58;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_state = 58;
                            continue;
                        }
                        65313 ... 65338 => {
                            __current_state = 58;
                            continue;
                        }
                        65343 => /* '\u{ff3f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        65345 ... 65370 => {
                            __current_state = 58;
                            continue;
                        }
                        65382 ... 65470 => {
                            __current_state = 58;
                            continue;
                        }
                        65474 ... 65479 => {
                            __current_state = 58;
                            continue;
                        }
                        65482 ... 65487 => {
                            __current_state = 58;
                            continue;
                        }
                        65490 ... 65495 => {
                            __current_state = 58;
                            continue;
                        }
                        65498 ... 65500 => {
                            __current_state = 58;
                            continue;
                        }
                        65536 ... 65547 => {
                            __current_state = 58;
                            continue;
                        }
                        65549 ... 65574 => {
                            __current_state = 58;
                            continue;
                        }
                        65576 ... 65594 => {
                            __current_state = 58;
                            continue;
                        }
                        65596 ... 65597 => {
                            __current_state = 58;
                            continue;
                        }
                        65599 ... 65613 => {
                            __current_state = 58;
                            continue;
                        }
                        65616 ... 65629 => {
                            __current_state = 58;
                            continue;
                        }
                        65664 ... 65786 => {
                            __current_state = 58;
                            continue;
                        }
                        65856 ... 65908 => {
                            __current_state = 58;
                            continue;
                        }
                        66045 => /* '\u{101fd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        66176 ... 66204 => {
                            __current_state = 58;
                            continue;
                        }
                        66208 ... 66256 => {
                            __current_state = 58;
                            continue;
                        }
                        66272 => /* '\u{102e0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        66304 ... 66335 => {
                            __current_state = 58;
                            continue;
                        }
                        66352 ... 66378 => {
                            __current_state = 58;
                            continue;
                        }
                        66384 ... 66426 => {
                            __current_state = 58;
                            continue;
                        }
                        66432 ... 66461 => {
                            __current_state = 58;
                            continue;
                        }
                        66464 ... 66499 => {
                            __current_state = 58;
                            continue;
                        }
                        66504 ... 66511 => {
                            __current_state = 58;
                            continue;
                        }
                        66513 ... 66517 => {
                            __current_state = 58;
                            continue;
                        }
                        66560 ... 66717 => {
                            __current_state = 58;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_state = 58;
                            continue;
                        }
                        66816 ... 66855 => {
                            __current_state = 58;
                            continue;
                        }
                        66864 ... 66915 => {
                            __current_state = 58;
                            continue;
                        }
                        67072 ... 67382 => {
                            __current_state = 58;
                            continue;
                        }
                        67392 ... 67413 => {
                            __current_state = 58;
                            continue;
                        }
                        67424 ... 67431 => {
                            __current_state = 58;
                            continue;
                        }
                        67584 ... 67589 => {
                            __current_state = 58;
                            continue;
                        }
                        67592 => /* '\u{10808}' */ {
                            __current_state = 58;
                            continue;
                        }
                        67594 ... 67637 => {
                            __current_state = 58;
                            continue;
                        }
                        67639 ... 67640 => {
                            __current_state = 58;
                            continue;
                        }
                        67644 => /* '\u{1083c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        67647 ... 67669 => {
                            __current_state = 58;
                            continue;
                        }
                        67680 ... 67702 => {
                            __current_state = 58;
                            continue;
                        }
                        67712 ... 67742 => {
                            __current_state = 58;
                            continue;
                        }
                        67808 ... 67826 => {
                            __current_state = 58;
                            continue;
                        }
                        67828 ... 67829 => {
                            __current_state = 58;
                            continue;
                        }
                        67840 ... 67861 => {
                            __current_state = 58;
                            continue;
                        }
                        67872 ... 67897 => {
                            __current_state = 58;
                            continue;
                        }
                        67968 ... 68023 => {
                            __current_state = 58;
                            continue;
                        }
                        68030 ... 68031 => {
                            __current_state = 58;
                            continue;
                        }
                        68096 ... 68099 => {
                            __current_state = 58;
                            continue;
                        }
                        68101 ... 68102 => {
                            __current_state = 58;
                            continue;
                        }
                        68108 ... 68115 => {
                            __current_state = 58;
                            continue;
                        }
                        68117 ... 68119 => {
                            __current_state = 58;
                            continue;
                        }
                        68121 ... 68147 => {
                            __current_state = 58;
                            continue;
                        }
                        68152 ... 68154 => {
                            __current_state = 58;
                            continue;
                        }
                        68159 => /* '\u{10a3f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        68192 ... 68220 => {
                            __current_state = 58;
                            continue;
                        }
                        68224 ... 68252 => {
                            __current_state = 58;
                            continue;
                        }
                        68288 ... 68295 => {
                            __current_state = 58;
                            continue;
                        }
                        68297 ... 68326 => {
                            __current_state = 58;
                            continue;
                        }
                        68352 ... 68405 => {
                            __current_state = 58;
                            continue;
                        }
                        68416 ... 68437 => {
                            __current_state = 58;
                            continue;
                        }
                        68448 ... 68466 => {
                            __current_state = 58;
                            continue;
                        }
                        68480 ... 68497 => {
                            __current_state = 58;
                            continue;
                        }
                        68608 ... 68680 => {
                            __current_state = 58;
                            continue;
                        }
                        68736 ... 68786 => {
                            __current_state = 58;
                            continue;
                        }
                        68800 ... 68850 => {
                            __current_state = 58;
                            continue;
                        }
                        69632 ... 69702 => {
                            __current_state = 58;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_state = 58;
                            continue;
                        }
                        69759 ... 69818 => {
                            __current_state = 58;
                            continue;
                        }
                        69840 ... 69864 => {
                            __current_state = 58;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_state = 58;
                            continue;
                        }
                        69888 ... 69940 => {
                            __current_state = 58;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_state = 58;
                            continue;
                        }
                        69968 ... 70003 => {
                            __current_state = 58;
                            continue;
                        }
                        70006 => /* '\u{11176}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70016 ... 70084 => {
                            __current_state = 58;
                            continue;
                        }
                        70090 ... 70092 => {
                            __current_state = 58;
                            continue;
                        }
                        70096 ... 70106 => {
                            __current_state = 58;
                            continue;
                        }
                        70108 => /* '\u{111dc}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70144 ... 70161 => {
                            __current_state = 58;
                            continue;
                        }
                        70163 ... 70199 => {
                            __current_state = 58;
                            continue;
                        }
                        70272 ... 70278 => {
                            __current_state = 58;
                            continue;
                        }
                        70280 => /* '\u{11288}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70282 ... 70285 => {
                            __current_state = 58;
                            continue;
                        }
                        70287 ... 70301 => {
                            __current_state = 58;
                            continue;
                        }
                        70303 ... 70312 => {
                            __current_state = 58;
                            continue;
                        }
                        70320 ... 70378 => {
                            __current_state = 58;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_state = 58;
                            continue;
                        }
                        70400 ... 70403 => {
                            __current_state = 58;
                            continue;
                        }
                        70405 ... 70412 => {
                            __current_state = 58;
                            continue;
                        }
                        70415 ... 70416 => {
                            __current_state = 58;
                            continue;
                        }
                        70419 ... 70440 => {
                            __current_state = 58;
                            continue;
                        }
                        70442 ... 70448 => {
                            __current_state = 58;
                            continue;
                        }
                        70450 ... 70451 => {
                            __current_state = 58;
                            continue;
                        }
                        70453 ... 70457 => {
                            __current_state = 58;
                            continue;
                        }
                        70460 ... 70468 => {
                            __current_state = 58;
                            continue;
                        }
                        70471 ... 70472 => {
                            __current_state = 58;
                            continue;
                        }
                        70475 ... 70477 => {
                            __current_state = 58;
                            continue;
                        }
                        70480 => /* '\u{11350}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70487 => /* '\u{11357}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70493 ... 70499 => {
                            __current_state = 58;
                            continue;
                        }
                        70502 ... 70508 => {
                            __current_state = 58;
                            continue;
                        }
                        70512 ... 70516 => {
                            __current_state = 58;
                            continue;
                        }
                        70784 ... 70853 => {
                            __current_state = 58;
                            continue;
                        }
                        70855 => /* '\u{114c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_state = 58;
                            continue;
                        }
                        71040 ... 71093 => {
                            __current_state = 58;
                            continue;
                        }
                        71096 ... 71104 => {
                            __current_state = 58;
                            continue;
                        }
                        71128 ... 71133 => {
                            __current_state = 58;
                            continue;
                        }
                        71168 ... 71232 => {
                            __current_state = 58;
                            continue;
                        }
                        71236 => /* '\u{11644}' */ {
                            __current_state = 58;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_state = 58;
                            continue;
                        }
                        71296 ... 71351 => {
                            __current_state = 58;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_state = 58;
                            continue;
                        }
                        71424 ... 71449 => {
                            __current_state = 58;
                            continue;
                        }
                        71453 ... 71467 => {
                            __current_state = 58;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_state = 58;
                            continue;
                        }
                        71840 ... 71913 => {
                            __current_state = 58;
                            continue;
                        }
                        71935 => /* '\u{118ff}' */ {
                            __current_state = 58;
                            continue;
                        }
                        72384 ... 72440 => {
                            __current_state = 58;
                            continue;
                        }
                        73728 ... 74649 => {
                            __current_state = 58;
                            continue;
                        }
                        74752 ... 74862 => {
                            __current_state = 58;
                            continue;
                        }
                        74880 ... 75075 => {
                            __current_state = 58;
                            continue;
                        }
                        77824 ... 78894 => {
                            __current_state = 58;
                            continue;
                        }
                        82944 ... 83526 => {
                            __current_state = 58;
                            continue;
                        }
                        92160 ... 92728 => {
                            __current_state = 58;
                            continue;
                        }
                        92736 ... 92766 => {
                            __current_state = 58;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_state = 58;
                            continue;
                        }
                        92880 ... 92909 => {
                            __current_state = 58;
                            continue;
                        }
                        92912 ... 92916 => {
                            __current_state = 58;
                            continue;
                        }
                        92928 ... 92982 => {
                            __current_state = 58;
                            continue;
                        }
                        92992 ... 92995 => {
                            __current_state = 58;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_state = 58;
                            continue;
                        }
                        93027 ... 93047 => {
                            __current_state = 58;
                            continue;
                        }
                        93053 ... 93071 => {
                            __current_state = 58;
                            continue;
                        }
                        93952 ... 94020 => {
                            __current_state = 58;
                            continue;
                        }
                        94032 ... 94078 => {
                            __current_state = 58;
                            continue;
                        }
                        94095 ... 94111 => {
                            __current_state = 58;
                            continue;
                        }
                        110592 ... 110593 => {
                            __current_state = 58;
                            continue;
                        }
                        113664 ... 113770 => {
                            __current_state = 58;
                            continue;
                        }
                        113776 ... 113788 => {
                            __current_state = 58;
                            continue;
                        }
                        113792 ... 113800 => {
                            __current_state = 58;
                            continue;
                        }
                        113808 ... 113817 => {
                            __current_state = 58;
                            continue;
                        }
                        113821 ... 113822 => {
                            __current_state = 58;
                            continue;
                        }
                        119141 ... 119145 => {
                            __current_state = 58;
                            continue;
                        }
                        119149 ... 119154 => {
                            __current_state = 58;
                            continue;
                        }
                        119163 ... 119170 => {
                            __current_state = 58;
                            continue;
                        }
                        119173 ... 119179 => {
                            __current_state = 58;
                            continue;
                        }
                        119210 ... 119213 => {
                            __current_state = 58;
                            continue;
                        }
                        119362 ... 119364 => {
                            __current_state = 58;
                            continue;
                        }
                        119808 ... 119892 => {
                            __current_state = 58;
                            continue;
                        }
                        119894 ... 119964 => {
                            __current_state = 58;
                            continue;
                        }
                        119966 ... 119967 => {
                            __current_state = 58;
                            continue;
                        }
                        119970 => /* '\u{1d4a2}' */ {
                            __current_state = 58;
                            continue;
                        }
                        119973 ... 119974 => {
                            __current_state = 58;
                            continue;
                        }
                        119977 ... 119980 => {
                            __current_state = 58;
                            continue;
                        }
                        119982 ... 119993 => {
                            __current_state = 58;
                            continue;
                        }
                        119995 => /* '\u{1d4bb}' */ {
                            __current_state = 58;
                            continue;
                        }
                        119997 ... 120003 => {
                            __current_state = 58;
                            continue;
                        }
                        120005 ... 120069 => {
                            __current_state = 58;
                            continue;
                        }
                        120071 ... 120074 => {
                            __current_state = 58;
                            continue;
                        }
                        120077 ... 120084 => {
                            __current_state = 58;
                            continue;
                        }
                        120086 ... 120092 => {
                            __current_state = 58;
                            continue;
                        }
                        120094 ... 120121 => {
                            __current_state = 58;
                            continue;
                        }
                        120123 ... 120126 => {
                            __current_state = 58;
                            continue;
                        }
                        120128 ... 120132 => {
                            __current_state = 58;
                            continue;
                        }
                        120134 => /* '\u{1d546}' */ {
                            __current_state = 58;
                            continue;
                        }
                        120138 ... 120144 => {
                            __current_state = 58;
                            continue;
                        }
                        120146 ... 120485 => {
                            __current_state = 58;
                            continue;
                        }
                        120488 ... 120512 => {
                            __current_state = 58;
                            continue;
                        }
                        120514 ... 120538 => {
                            __current_state = 58;
                            continue;
                        }
                        120540 ... 120570 => {
                            __current_state = 58;
                            continue;
                        }
                        120572 ... 120596 => {
                            __current_state = 58;
                            continue;
                        }
                        120598 ... 120628 => {
                            __current_state = 58;
                            continue;
                        }
                        120630 ... 120654 => {
                            __current_state = 58;
                            continue;
                        }
                        120656 ... 120686 => {
                            __current_state = 58;
                            continue;
                        }
                        120688 ... 120712 => {
                            __current_state = 58;
                            continue;
                        }
                        120714 ... 120744 => {
                            __current_state = 58;
                            continue;
                        }
                        120746 ... 120770 => {
                            __current_state = 58;
                            continue;
                        }
                        120772 ... 120779 => {
                            __current_state = 58;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_state = 58;
                            continue;
                        }
                        121344 ... 121398 => {
                            __current_state = 58;
                            continue;
                        }
                        121403 ... 121452 => {
                            __current_state = 58;
                            continue;
                        }
                        121461 => /* '\u{1da75}' */ {
                            __current_state = 58;
                            continue;
                        }
                        121476 => /* '\u{1da84}' */ {
                            __current_state = 58;
                            continue;
                        }
                        121499 ... 121503 => {
                            __current_state = 58;
                            continue;
                        }
                        121505 ... 121519 => {
                            __current_state = 58;
                            continue;
                        }
                        124928 ... 125124 => {
                            __current_state = 58;
                            continue;
                        }
                        125136 ... 125142 => {
                            __current_state = 58;
                            continue;
                        }
                        126464 ... 126467 => {
                            __current_state = 58;
                            continue;
                        }
                        126469 ... 126495 => {
                            __current_state = 58;
                            continue;
                        }
                        126497 ... 126498 => {
                            __current_state = 58;
                            continue;
                        }
                        126500 => /* '\u{1ee24}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126503 => /* '\u{1ee27}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126505 ... 126514 => {
                            __current_state = 58;
                            continue;
                        }
                        126516 ... 126519 => {
                            __current_state = 58;
                            continue;
                        }
                        126521 => /* '\u{1ee39}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126523 => /* '\u{1ee3b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126530 => /* '\u{1ee42}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126535 => /* '\u{1ee47}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126537 => /* '\u{1ee49}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126539 => /* '\u{1ee4b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126541 ... 126543 => {
                            __current_state = 58;
                            continue;
                        }
                        126545 ... 126546 => {
                            __current_state = 58;
                            continue;
                        }
                        126548 => /* '\u{1ee54}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126551 => /* '\u{1ee57}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126553 => /* '\u{1ee59}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126555 => /* '\u{1ee5b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126557 => /* '\u{1ee5d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126559 => /* '\u{1ee5f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126561 ... 126562 => {
                            __current_state = 58;
                            continue;
                        }
                        126564 => /* '\u{1ee64}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126567 ... 126570 => {
                            __current_state = 58;
                            continue;
                        }
                        126572 ... 126578 => {
                            __current_state = 58;
                            continue;
                        }
                        126580 ... 126583 => {
                            __current_state = 58;
                            continue;
                        }
                        126585 ... 126588 => {
                            __current_state = 58;
                            continue;
                        }
                        126590 => /* '\u{1ee7e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126592 ... 126601 => {
                            __current_state = 58;
                            continue;
                        }
                        126603 ... 126619 => {
                            __current_state = 58;
                            continue;
                        }
                        126625 ... 126627 => {
                            __current_state = 58;
                            continue;
                        }
                        126629 ... 126633 => {
                            __current_state = 58;
                            continue;
                        }
                        126635 ... 126651 => {
                            __current_state = 58;
                            continue;
                        }
                        127280 ... 127305 => {
                            __current_state = 58;
                            continue;
                        }
                        127312 ... 127337 => {
                            __current_state = 58;
                            continue;
                        }
                        127344 ... 127369 => {
                            __current_state = 58;
                            continue;
                        }
                        131072 ... 173782 => {
                            __current_state = 58;
                            continue;
                        }
                        173824 ... 177972 => {
                            __current_state = 58;
                            continue;
                        }
                        177984 ... 178205 => {
                            __current_state = 58;
                            continue;
                        }
                        178208 ... 183969 => {
                            __current_state = 58;
                            continue;
                        }
                        194560 ... 195101 => {
                            __current_state = 58;
                            continue;
                        }
                        917760 ... 917999 => {
                            __current_state = 58;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 59;
                            continue;
                        }
                        _ => {
                            __current_state = 60;
                            continue;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        _ => {
                            __current_match = Some((9, __index + __ch.len_utf8()));
                            __current_state = 26;
                            continue;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((44, __index + __ch.len_utf8()));
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 61;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 61;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 62;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 62;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 63;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 63;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 64;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 64;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 65;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 65;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 66;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        80 => /* 'P' */ {
                            __current_state = 67;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 68;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 67;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 68;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        71 => /* 'G' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 69;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 69;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        67 => /* 'C' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 70;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 70;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_match = Some((36, __index + 1));
                            __current_state = 71;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((36, __index + 1));
                            __current_state = 71;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 72;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 72;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        66 => /* 'B' */ {
                            __current_match = Some((19, __index + 1));
                            __current_state = 73;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((19, __index + 1));
                            __current_state = 73;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 74;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 74;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((21, __index + 1));
                            __current_state = 75;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((21, __index + 1));
                            __current_state = 75;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 76;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 77;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 76;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 77;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_match = Some((24, __index + 1));
                            __current_state = 78;
                            continue;
                        }
                        89 => /* 'Y' */ {
                            __current_match = Some((25, __index + 1));
                            __current_state = 79;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((24, __index + 1));
                            __current_state = 78;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((25, __index + 1));
                            __current_state = 79;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((26, __index + 1));
                            __current_state = 80;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((26, __index + 1));
                            __current_state = 80;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        86 => /* 'V' */ {
                            __current_match = Some((27, __index + 1));
                            __current_state = 81;
                            continue;
                        }
                        118 => /* 'v' */ {
                            __current_match = Some((27, __index + 1));
                            __current_state = 81;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((28, __index + 1));
                            __current_state = 82;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((28, __index + 1));
                            __current_state = 82;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        83 => /* 'S' */ {
                            __current_state = 83;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 83;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((29, __index + 1));
                            __current_state = 84;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((29, __index + 1));
                            __current_state = 84;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        80 => /* 'P' */ {
                            __current_match = Some((30, __index + 1));
                            __current_state = 85;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((30, __index + 1));
                            __current_state = 85;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((31, __index + 1));
                            __current_state = 86;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((31, __index + 1));
                            __current_state = 86;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        88 => /* 'X' */ {
                            __current_state = 87;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_state = 87;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        85 => /* 'U' */ {
                            __current_match = Some((32, __index + 1));
                            __current_state = 88;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((32, __index + 1));
                            __current_state = 88;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_match = Some((33, __index + 1));
                            __current_state = 89;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((33, __index + 1));
                            __current_state = 89;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        68 => /* 'D' */ {
                            __current_match = Some((34, __index + 1));
                            __current_state = 90;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((34, __index + 1));
                            __current_state = 90;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 91;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_state = 58;
                            continue;
                        }
                        48 ... 57 => {
                            __current_state = 58;
                            continue;
                        }
                        64 ... 90 => {
                            __current_state = 58;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_state = 58;
                            continue;
                        }
                        97 ... 122 => {
                            __current_state = 58;
                            continue;
                        }
                        170 => /* '\u{aa}' */ {
                            __current_state = 58;
                            continue;
                        }
                        181 => /* '\u{b5}' */ {
                            __current_state = 58;
                            continue;
                        }
                        186 => /* '\u{ba}' */ {
                            __current_state = 58;
                            continue;
                        }
                        192 ... 214 => {
                            __current_state = 58;
                            continue;
                        }
                        216 ... 246 => {
                            __current_state = 58;
                            continue;
                        }
                        248 ... 705 => {
                            __current_state = 58;
                            continue;
                        }
                        710 ... 721 => {
                            __current_state = 58;
                            continue;
                        }
                        736 ... 740 => {
                            __current_state = 58;
                            continue;
                        }
                        748 => /* '\u{2ec}' */ {
                            __current_state = 58;
                            continue;
                        }
                        750 => /* '\u{2ee}' */ {
                            __current_state = 58;
                            continue;
                        }
                        768 ... 884 => {
                            __current_state = 58;
                            continue;
                        }
                        886 ... 887 => {
                            __current_state = 58;
                            continue;
                        }
                        890 ... 893 => {
                            __current_state = 58;
                            continue;
                        }
                        895 => /* '\u{37f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        902 => /* '\u{386}' */ {
                            __current_state = 58;
                            continue;
                        }
                        904 ... 906 => {
                            __current_state = 58;
                            continue;
                        }
                        908 => /* '\u{38c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        910 ... 929 => {
                            __current_state = 58;
                            continue;
                        }
                        931 ... 1013 => {
                            __current_state = 58;
                            continue;
                        }
                        1015 ... 1153 => {
                            __current_state = 58;
                            continue;
                        }
                        1155 ... 1327 => {
                            __current_state = 58;
                            continue;
                        }
                        1329 ... 1366 => {
                            __current_state = 58;
                            continue;
                        }
                        1369 => /* '\u{559}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1377 ... 1415 => {
                            __current_state = 58;
                            continue;
                        }
                        1425 ... 1469 => {
                            __current_state = 58;
                            continue;
                        }
                        1471 => /* '\u{5bf}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1473 ... 1474 => {
                            __current_state = 58;
                            continue;
                        }
                        1476 ... 1477 => {
                            __current_state = 58;
                            continue;
                        }
                        1479 => /* '\u{5c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1488 ... 1514 => {
                            __current_state = 58;
                            continue;
                        }
                        1520 ... 1522 => {
                            __current_state = 58;
                            continue;
                        }
                        1552 ... 1562 => {
                            __current_state = 58;
                            continue;
                        }
                        1568 ... 1641 => {
                            __current_state = 58;
                            continue;
                        }
                        1646 ... 1747 => {
                            __current_state = 58;
                            continue;
                        }
                        1749 ... 1756 => {
                            __current_state = 58;
                            continue;
                        }
                        1759 ... 1768 => {
                            __current_state = 58;
                            continue;
                        }
                        1770 ... 1788 => {
                            __current_state = 58;
                            continue;
                        }
                        1791 => /* '\u{6ff}' */ {
                            __current_state = 58;
                            continue;
                        }
                        1808 ... 1866 => {
                            __current_state = 58;
                            continue;
                        }
                        1869 ... 1969 => {
                            __current_state = 58;
                            continue;
                        }
                        1984 ... 2037 => {
                            __current_state = 58;
                            continue;
                        }
                        2042 => /* '\u{7fa}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2048 ... 2093 => {
                            __current_state = 58;
                            continue;
                        }
                        2112 ... 2139 => {
                            __current_state = 58;
                            continue;
                        }
                        2208 ... 2228 => {
                            __current_state = 58;
                            continue;
                        }
                        2275 ... 2403 => {
                            __current_state = 58;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_state = 58;
                            continue;
                        }
                        2417 ... 2435 => {
                            __current_state = 58;
                            continue;
                        }
                        2437 ... 2444 => {
                            __current_state = 58;
                            continue;
                        }
                        2447 ... 2448 => {
                            __current_state = 58;
                            continue;
                        }
                        2451 ... 2472 => {
                            __current_state = 58;
                            continue;
                        }
                        2474 ... 2480 => {
                            __current_state = 58;
                            continue;
                        }
                        2482 => /* '\u{9b2}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2486 ... 2489 => {
                            __current_state = 58;
                            continue;
                        }
                        2492 ... 2500 => {
                            __current_state = 58;
                            continue;
                        }
                        2503 ... 2504 => {
                            __current_state = 58;
                            continue;
                        }
                        2507 ... 2510 => {
                            __current_state = 58;
                            continue;
                        }
                        2519 => /* '\u{9d7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2524 ... 2525 => {
                            __current_state = 58;
                            continue;
                        }
                        2527 ... 2531 => {
                            __current_state = 58;
                            continue;
                        }
                        2534 ... 2545 => {
                            __current_state = 58;
                            continue;
                        }
                        2561 ... 2563 => {
                            __current_state = 58;
                            continue;
                        }
                        2565 ... 2570 => {
                            __current_state = 58;
                            continue;
                        }
                        2575 ... 2576 => {
                            __current_state = 58;
                            continue;
                        }
                        2579 ... 2600 => {
                            __current_state = 58;
                            continue;
                        }
                        2602 ... 2608 => {
                            __current_state = 58;
                            continue;
                        }
                        2610 ... 2611 => {
                            __current_state = 58;
                            continue;
                        }
                        2613 ... 2614 => {
                            __current_state = 58;
                            continue;
                        }
                        2616 ... 2617 => {
                            __current_state = 58;
                            continue;
                        }
                        2620 => /* '\u{a3c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2622 ... 2626 => {
                            __current_state = 58;
                            continue;
                        }
                        2631 ... 2632 => {
                            __current_state = 58;
                            continue;
                        }
                        2635 ... 2637 => {
                            __current_state = 58;
                            continue;
                        }
                        2641 => /* '\u{a51}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2649 ... 2652 => {
                            __current_state = 58;
                            continue;
                        }
                        2654 => /* '\u{a5e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2662 ... 2677 => {
                            __current_state = 58;
                            continue;
                        }
                        2689 ... 2691 => {
                            __current_state = 58;
                            continue;
                        }
                        2693 ... 2701 => {
                            __current_state = 58;
                            continue;
                        }
                        2703 ... 2705 => {
                            __current_state = 58;
                            continue;
                        }
                        2707 ... 2728 => {
                            __current_state = 58;
                            continue;
                        }
                        2730 ... 2736 => {
                            __current_state = 58;
                            continue;
                        }
                        2738 ... 2739 => {
                            __current_state = 58;
                            continue;
                        }
                        2741 ... 2745 => {
                            __current_state = 58;
                            continue;
                        }
                        2748 ... 2757 => {
                            __current_state = 58;
                            continue;
                        }
                        2759 ... 2761 => {
                            __current_state = 58;
                            continue;
                        }
                        2763 ... 2765 => {
                            __current_state = 58;
                            continue;
                        }
                        2768 => /* '\u{ad0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2784 ... 2787 => {
                            __current_state = 58;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_state = 58;
                            continue;
                        }
                        2809 => /* '\u{af9}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2817 ... 2819 => {
                            __current_state = 58;
                            continue;
                        }
                        2821 ... 2828 => {
                            __current_state = 58;
                            continue;
                        }
                        2831 ... 2832 => {
                            __current_state = 58;
                            continue;
                        }
                        2835 ... 2856 => {
                            __current_state = 58;
                            continue;
                        }
                        2858 ... 2864 => {
                            __current_state = 58;
                            continue;
                        }
                        2866 ... 2867 => {
                            __current_state = 58;
                            continue;
                        }
                        2869 ... 2873 => {
                            __current_state = 58;
                            continue;
                        }
                        2876 ... 2884 => {
                            __current_state = 58;
                            continue;
                        }
                        2887 ... 2888 => {
                            __current_state = 58;
                            continue;
                        }
                        2891 ... 2893 => {
                            __current_state = 58;
                            continue;
                        }
                        2902 ... 2903 => {
                            __current_state = 58;
                            continue;
                        }
                        2908 ... 2909 => {
                            __current_state = 58;
                            continue;
                        }
                        2911 ... 2915 => {
                            __current_state = 58;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_state = 58;
                            continue;
                        }
                        2929 => /* '\u{b71}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2946 ... 2947 => {
                            __current_state = 58;
                            continue;
                        }
                        2949 ... 2954 => {
                            __current_state = 58;
                            continue;
                        }
                        2958 ... 2960 => {
                            __current_state = 58;
                            continue;
                        }
                        2962 ... 2965 => {
                            __current_state = 58;
                            continue;
                        }
                        2969 ... 2970 => {
                            __current_state = 58;
                            continue;
                        }
                        2972 => /* '\u{b9c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        2974 ... 2975 => {
                            __current_state = 58;
                            continue;
                        }
                        2979 ... 2980 => {
                            __current_state = 58;
                            continue;
                        }
                        2984 ... 2986 => {
                            __current_state = 58;
                            continue;
                        }
                        2990 ... 3001 => {
                            __current_state = 58;
                            continue;
                        }
                        3006 ... 3010 => {
                            __current_state = 58;
                            continue;
                        }
                        3014 ... 3016 => {
                            __current_state = 58;
                            continue;
                        }
                        3018 ... 3021 => {
                            __current_state = 58;
                            continue;
                        }
                        3024 => /* '\u{bd0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3031 => /* '\u{bd7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_state = 58;
                            continue;
                        }
                        3072 ... 3075 => {
                            __current_state = 58;
                            continue;
                        }
                        3077 ... 3084 => {
                            __current_state = 58;
                            continue;
                        }
                        3086 ... 3088 => {
                            __current_state = 58;
                            continue;
                        }
                        3090 ... 3112 => {
                            __current_state = 58;
                            continue;
                        }
                        3114 ... 3129 => {
                            __current_state = 58;
                            continue;
                        }
                        3133 ... 3140 => {
                            __current_state = 58;
                            continue;
                        }
                        3142 ... 3144 => {
                            __current_state = 58;
                            continue;
                        }
                        3146 ... 3149 => {
                            __current_state = 58;
                            continue;
                        }
                        3157 ... 3158 => {
                            __current_state = 58;
                            continue;
                        }
                        3160 ... 3162 => {
                            __current_state = 58;
                            continue;
                        }
                        3168 ... 3171 => {
                            __current_state = 58;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_state = 58;
                            continue;
                        }
                        3201 ... 3203 => {
                            __current_state = 58;
                            continue;
                        }
                        3205 ... 3212 => {
                            __current_state = 58;
                            continue;
                        }
                        3214 ... 3216 => {
                            __current_state = 58;
                            continue;
                        }
                        3218 ... 3240 => {
                            __current_state = 58;
                            continue;
                        }
                        3242 ... 3251 => {
                            __current_state = 58;
                            continue;
                        }
                        3253 ... 3257 => {
                            __current_state = 58;
                            continue;
                        }
                        3260 ... 3268 => {
                            __current_state = 58;
                            continue;
                        }
                        3270 ... 3272 => {
                            __current_state = 58;
                            continue;
                        }
                        3274 ... 3277 => {
                            __current_state = 58;
                            continue;
                        }
                        3285 ... 3286 => {
                            __current_state = 58;
                            continue;
                        }
                        3294 => /* '\u{cde}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3296 ... 3299 => {
                            __current_state = 58;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_state = 58;
                            continue;
                        }
                        3313 ... 3314 => {
                            __current_state = 58;
                            continue;
                        }
                        3329 ... 3331 => {
                            __current_state = 58;
                            continue;
                        }
                        3333 ... 3340 => {
                            __current_state = 58;
                            continue;
                        }
                        3342 ... 3344 => {
                            __current_state = 58;
                            continue;
                        }
                        3346 ... 3386 => {
                            __current_state = 58;
                            continue;
                        }
                        3389 ... 3396 => {
                            __current_state = 58;
                            continue;
                        }
                        3398 ... 3400 => {
                            __current_state = 58;
                            continue;
                        }
                        3402 ... 3406 => {
                            __current_state = 58;
                            continue;
                        }
                        3415 => /* '\u{d57}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3423 ... 3427 => {
                            __current_state = 58;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_state = 58;
                            continue;
                        }
                        3450 ... 3455 => {
                            __current_state = 58;
                            continue;
                        }
                        3458 ... 3459 => {
                            __current_state = 58;
                            continue;
                        }
                        3461 ... 3478 => {
                            __current_state = 58;
                            continue;
                        }
                        3482 ... 3505 => {
                            __current_state = 58;
                            continue;
                        }
                        3507 ... 3515 => {
                            __current_state = 58;
                            continue;
                        }
                        3517 => /* '\u{dbd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3520 ... 3526 => {
                            __current_state = 58;
                            continue;
                        }
                        3530 => /* '\u{dca}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3535 ... 3540 => {
                            __current_state = 58;
                            continue;
                        }
                        3542 => /* '\u{dd6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3544 ... 3551 => {
                            __current_state = 58;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_state = 58;
                            continue;
                        }
                        3570 ... 3571 => {
                            __current_state = 58;
                            continue;
                        }
                        3585 ... 3642 => {
                            __current_state = 58;
                            continue;
                        }
                        3648 ... 3662 => {
                            __current_state = 58;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_state = 58;
                            continue;
                        }
                        3713 ... 3714 => {
                            __current_state = 58;
                            continue;
                        }
                        3716 => /* '\u{e84}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3719 ... 3720 => {
                            __current_state = 58;
                            continue;
                        }
                        3722 => /* '\u{e8a}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3725 => /* '\u{e8d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3732 ... 3735 => {
                            __current_state = 58;
                            continue;
                        }
                        3737 ... 3743 => {
                            __current_state = 58;
                            continue;
                        }
                        3745 ... 3747 => {
                            __current_state = 58;
                            continue;
                        }
                        3749 => /* '\u{ea5}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3751 => /* '\u{ea7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3754 ... 3755 => {
                            __current_state = 58;
                            continue;
                        }
                        3757 ... 3769 => {
                            __current_state = 58;
                            continue;
                        }
                        3771 ... 3773 => {
                            __current_state = 58;
                            continue;
                        }
                        3776 ... 3780 => {
                            __current_state = 58;
                            continue;
                        }
                        3782 => /* '\u{ec6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3784 ... 3789 => {
                            __current_state = 58;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_state = 58;
                            continue;
                        }
                        3804 ... 3807 => {
                            __current_state = 58;
                            continue;
                        }
                        3840 => /* '\u{f00}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3864 ... 3865 => {
                            __current_state = 58;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_state = 58;
                            continue;
                        }
                        3893 => /* '\u{f35}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3895 => /* '\u{f37}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3897 => /* '\u{f39}' */ {
                            __current_state = 58;
                            continue;
                        }
                        3902 ... 3911 => {
                            __current_state = 58;
                            continue;
                        }
                        3913 ... 3948 => {
                            __current_state = 58;
                            continue;
                        }
                        3953 ... 3972 => {
                            __current_state = 58;
                            continue;
                        }
                        3974 ... 3991 => {
                            __current_state = 58;
                            continue;
                        }
                        3993 ... 4028 => {
                            __current_state = 58;
                            continue;
                        }
                        4038 => /* '\u{fc6}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4096 ... 4169 => {
                            __current_state = 58;
                            continue;
                        }
                        4176 ... 4253 => {
                            __current_state = 58;
                            continue;
                        }
                        4256 ... 4293 => {
                            __current_state = 58;
                            continue;
                        }
                        4295 => /* '\u{10c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4301 => /* '\u{10cd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4304 ... 4346 => {
                            __current_state = 58;
                            continue;
                        }
                        4348 ... 4680 => {
                            __current_state = 58;
                            continue;
                        }
                        4682 ... 4685 => {
                            __current_state = 58;
                            continue;
                        }
                        4688 ... 4694 => {
                            __current_state = 58;
                            continue;
                        }
                        4696 => /* '\u{1258}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4698 ... 4701 => {
                            __current_state = 58;
                            continue;
                        }
                        4704 ... 4744 => {
                            __current_state = 58;
                            continue;
                        }
                        4746 ... 4749 => {
                            __current_state = 58;
                            continue;
                        }
                        4752 ... 4784 => {
                            __current_state = 58;
                            continue;
                        }
                        4786 ... 4789 => {
                            __current_state = 58;
                            continue;
                        }
                        4792 ... 4798 => {
                            __current_state = 58;
                            continue;
                        }
                        4800 => /* '\u{12c0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        4802 ... 4805 => {
                            __current_state = 58;
                            continue;
                        }
                        4808 ... 4822 => {
                            __current_state = 58;
                            continue;
                        }
                        4824 ... 4880 => {
                            __current_state = 58;
                            continue;
                        }
                        4882 ... 4885 => {
                            __current_state = 58;
                            continue;
                        }
                        4888 ... 4954 => {
                            __current_state = 58;
                            continue;
                        }
                        4957 ... 4959 => {
                            __current_state = 58;
                            continue;
                        }
                        4992 ... 5007 => {
                            __current_state = 58;
                            continue;
                        }
                        5024 ... 5109 => {
                            __current_state = 58;
                            continue;
                        }
                        5112 ... 5117 => {
                            __current_state = 58;
                            continue;
                        }
                        5121 ... 5740 => {
                            __current_state = 58;
                            continue;
                        }
                        5743 ... 5759 => {
                            __current_state = 58;
                            continue;
                        }
                        5761 ... 5786 => {
                            __current_state = 58;
                            continue;
                        }
                        5792 ... 5866 => {
                            __current_state = 58;
                            continue;
                        }
                        5870 ... 5880 => {
                            __current_state = 58;
                            continue;
                        }
                        5888 ... 5900 => {
                            __current_state = 58;
                            continue;
                        }
                        5902 ... 5908 => {
                            __current_state = 58;
                            continue;
                        }
                        5920 ... 5940 => {
                            __current_state = 58;
                            continue;
                        }
                        5952 ... 5971 => {
                            __current_state = 58;
                            continue;
                        }
                        5984 ... 5996 => {
                            __current_state = 58;
                            continue;
                        }
                        5998 ... 6000 => {
                            __current_state = 58;
                            continue;
                        }
                        6002 ... 6003 => {
                            __current_state = 58;
                            continue;
                        }
                        6016 ... 6099 => {
                            __current_state = 58;
                            continue;
                        }
                        6103 => /* '\u{17d7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        6108 ... 6109 => {
                            __current_state = 58;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_state = 58;
                            continue;
                        }
                        6155 ... 6157 => {
                            __current_state = 58;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_state = 58;
                            continue;
                        }
                        6176 ... 6263 => {
                            __current_state = 58;
                            continue;
                        }
                        6272 ... 6314 => {
                            __current_state = 58;
                            continue;
                        }
                        6320 ... 6389 => {
                            __current_state = 58;
                            continue;
                        }
                        6400 ... 6430 => {
                            __current_state = 58;
                            continue;
                        }
                        6432 ... 6443 => {
                            __current_state = 58;
                            continue;
                        }
                        6448 ... 6459 => {
                            __current_state = 58;
                            continue;
                        }
                        6470 ... 6509 => {
                            __current_state = 58;
                            continue;
                        }
                        6512 ... 6516 => {
                            __current_state = 58;
                            continue;
                        }
                        6528 ... 6571 => {
                            __current_state = 58;
                            continue;
                        }
                        6576 ... 6601 => {
                            __current_state = 58;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_state = 58;
                            continue;
                        }
                        6656 ... 6683 => {
                            __current_state = 58;
                            continue;
                        }
                        6688 ... 6750 => {
                            __current_state = 58;
                            continue;
                        }
                        6752 ... 6780 => {
                            __current_state = 58;
                            continue;
                        }
                        6783 ... 6793 => {
                            __current_state = 58;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_state = 58;
                            continue;
                        }
                        6823 => /* '\u{1aa7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        6832 ... 6846 => {
                            __current_state = 58;
                            continue;
                        }
                        6912 ... 6987 => {
                            __current_state = 58;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_state = 58;
                            continue;
                        }
                        7019 ... 7027 => {
                            __current_state = 58;
                            continue;
                        }
                        7040 ... 7155 => {
                            __current_state = 58;
                            continue;
                        }
                        7168 ... 7223 => {
                            __current_state = 58;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_state = 58;
                            continue;
                        }
                        7245 ... 7293 => {
                            __current_state = 58;
                            continue;
                        }
                        7376 ... 7378 => {
                            __current_state = 58;
                            continue;
                        }
                        7380 ... 7414 => {
                            __current_state = 58;
                            continue;
                        }
                        7416 ... 7417 => {
                            __current_state = 58;
                            continue;
                        }
                        7424 ... 7669 => {
                            __current_state = 58;
                            continue;
                        }
                        7676 ... 7957 => {
                            __current_state = 58;
                            continue;
                        }
                        7960 ... 7965 => {
                            __current_state = 58;
                            continue;
                        }
                        7968 ... 8005 => {
                            __current_state = 58;
                            continue;
                        }
                        8008 ... 8013 => {
                            __current_state = 58;
                            continue;
                        }
                        8016 ... 8023 => {
                            __current_state = 58;
                            continue;
                        }
                        8025 => /* '\u{1f59}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8027 => /* '\u{1f5b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8029 => /* '\u{1f5d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8031 ... 8061 => {
                            __current_state = 58;
                            continue;
                        }
                        8064 ... 8116 => {
                            __current_state = 58;
                            continue;
                        }
                        8118 ... 8124 => {
                            __current_state = 58;
                            continue;
                        }
                        8126 => /* '\u{1fbe}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8130 ... 8132 => {
                            __current_state = 58;
                            continue;
                        }
                        8134 ... 8140 => {
                            __current_state = 58;
                            continue;
                        }
                        8144 ... 8147 => {
                            __current_state = 58;
                            continue;
                        }
                        8150 ... 8155 => {
                            __current_state = 58;
                            continue;
                        }
                        8160 ... 8172 => {
                            __current_state = 58;
                            continue;
                        }
                        8178 ... 8180 => {
                            __current_state = 58;
                            continue;
                        }
                        8182 ... 8188 => {
                            __current_state = 58;
                            continue;
                        }
                        8204 ... 8205 => {
                            __current_state = 58;
                            continue;
                        }
                        8255 ... 8256 => {
                            __current_state = 58;
                            continue;
                        }
                        8276 => /* '\u{2054}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8305 => /* '\u{2071}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8319 => /* '\u{207f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8336 ... 8348 => {
                            __current_state = 58;
                            continue;
                        }
                        8400 ... 8432 => {
                            __current_state = 58;
                            continue;
                        }
                        8450 => /* '\u{2102}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8455 => /* '\u{2107}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8458 ... 8467 => {
                            __current_state = 58;
                            continue;
                        }
                        8469 => /* '\u{2115}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8473 ... 8477 => {
                            __current_state = 58;
                            continue;
                        }
                        8484 => /* '\u{2124}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8486 => /* '\u{2126}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8488 => /* '\u{2128}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8490 ... 8493 => {
                            __current_state = 58;
                            continue;
                        }
                        8495 ... 8505 => {
                            __current_state = 58;
                            continue;
                        }
                        8508 ... 8511 => {
                            __current_state = 58;
                            continue;
                        }
                        8517 ... 8521 => {
                            __current_state = 58;
                            continue;
                        }
                        8526 => /* '\u{214e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        8544 ... 8584 => {
                            __current_state = 58;
                            continue;
                        }
                        9398 ... 9449 => {
                            __current_state = 58;
                            continue;
                        }
                        11264 ... 11310 => {
                            __current_state = 58;
                            continue;
                        }
                        11312 ... 11358 => {
                            __current_state = 58;
                            continue;
                        }
                        11360 ... 11492 => {
                            __current_state = 58;
                            continue;
                        }
                        11499 ... 11507 => {
                            __current_state = 58;
                            continue;
                        }
                        11520 ... 11557 => {
                            __current_state = 58;
                            continue;
                        }
                        11559 => /* '\u{2d27}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11565 => /* '\u{2d2d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11568 ... 11623 => {
                            __current_state = 58;
                            continue;
                        }
                        11631 => /* '\u{2d6f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        11647 ... 11670 => {
                            __current_state = 58;
                            continue;
                        }
                        11680 ... 11686 => {
                            __current_state = 58;
                            continue;
                        }
                        11688 ... 11694 => {
                            __current_state = 58;
                            continue;
                        }
                        11696 ... 11702 => {
                            __current_state = 58;
                            continue;
                        }
                        11704 ... 11710 => {
                            __current_state = 58;
                            continue;
                        }
                        11712 ... 11718 => {
                            __current_state = 58;
                            continue;
                        }
                        11720 ... 11726 => {
                            __current_state = 58;
                            continue;
                        }
                        11728 ... 11734 => {
                            __current_state = 58;
                            continue;
                        }
                        11736 ... 11742 => {
                            __current_state = 58;
                            continue;
                        }
                        11744 ... 11775 => {
                            __current_state = 58;
                            continue;
                        }
                        11823 => /* '\u{2e2f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        12293 ... 12295 => {
                            __current_state = 58;
                            continue;
                        }
                        12321 ... 12335 => {
                            __current_state = 58;
                            continue;
                        }
                        12337 ... 12341 => {
                            __current_state = 58;
                            continue;
                        }
                        12344 ... 12348 => {
                            __current_state = 58;
                            continue;
                        }
                        12353 ... 12438 => {
                            __current_state = 58;
                            continue;
                        }
                        12441 ... 12442 => {
                            __current_state = 58;
                            continue;
                        }
                        12445 ... 12447 => {
                            __current_state = 58;
                            continue;
                        }
                        12449 ... 12538 => {
                            __current_state = 58;
                            continue;
                        }
                        12540 ... 12543 => {
                            __current_state = 58;
                            continue;
                        }
                        12549 ... 12589 => {
                            __current_state = 58;
                            continue;
                        }
                        12593 ... 12686 => {
                            __current_state = 58;
                            continue;
                        }
                        12704 ... 12730 => {
                            __current_state = 58;
                            continue;
                        }
                        12784 ... 12799 => {
                            __current_state = 58;
                            continue;
                        }
                        13312 ... 19893 => {
                            __current_state = 58;
                            continue;
                        }
                        19968 ... 40917 => {
                            __current_state = 58;
                            continue;
                        }
                        40960 ... 42124 => {
                            __current_state = 58;
                            continue;
                        }
                        42192 ... 42237 => {
                            __current_state = 58;
                            continue;
                        }
                        42240 ... 42508 => {
                            __current_state = 58;
                            continue;
                        }
                        42512 ... 42539 => {
                            __current_state = 58;
                            continue;
                        }
                        42560 ... 42610 => {
                            __current_state = 58;
                            continue;
                        }
                        42612 ... 42621 => {
                            __current_state = 58;
                            continue;
                        }
                        42623 ... 42737 => {
                            __current_state = 58;
                            continue;
                        }
                        42775 ... 42783 => {
                            __current_state = 58;
                            continue;
                        }
                        42786 ... 42888 => {
                            __current_state = 58;
                            continue;
                        }
                        42891 ... 42925 => {
                            __current_state = 58;
                            continue;
                        }
                        42928 ... 42935 => {
                            __current_state = 58;
                            continue;
                        }
                        42999 ... 43047 => {
                            __current_state = 58;
                            continue;
                        }
                        43072 ... 43123 => {
                            __current_state = 58;
                            continue;
                        }
                        43136 ... 43204 => {
                            __current_state = 58;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_state = 58;
                            continue;
                        }
                        43232 ... 43255 => {
                            __current_state = 58;
                            continue;
                        }
                        43259 => /* '\u{a8fb}' */ {
                            __current_state = 58;
                            continue;
                        }
                        43261 => /* '\u{a8fd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        43264 ... 43309 => {
                            __current_state = 58;
                            continue;
                        }
                        43312 ... 43347 => {
                            __current_state = 58;
                            continue;
                        }
                        43360 ... 43388 => {
                            __current_state = 58;
                            continue;
                        }
                        43392 ... 43456 => {
                            __current_state = 58;
                            continue;
                        }
                        43471 ... 43481 => {
                            __current_state = 58;
                            continue;
                        }
                        43488 ... 43518 => {
                            __current_state = 58;
                            continue;
                        }
                        43520 ... 43574 => {
                            __current_state = 58;
                            continue;
                        }
                        43584 ... 43597 => {
                            __current_state = 58;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_state = 58;
                            continue;
                        }
                        43616 ... 43638 => {
                            __current_state = 58;
                            continue;
                        }
                        43642 ... 43714 => {
                            __current_state = 58;
                            continue;
                        }
                        43739 ... 43741 => {
                            __current_state = 58;
                            continue;
                        }
                        43744 ... 43759 => {
                            __current_state = 58;
                            continue;
                        }
                        43762 ... 43766 => {
                            __current_state = 58;
                            continue;
                        }
                        43777 ... 43782 => {
                            __current_state = 58;
                            continue;
                        }
                        43785 ... 43790 => {
                            __current_state = 58;
                            continue;
                        }
                        43793 ... 43798 => {
                            __current_state = 58;
                            continue;
                        }
                        43808 ... 43814 => {
                            __current_state = 58;
                            continue;
                        }
                        43816 ... 43822 => {
                            __current_state = 58;
                            continue;
                        }
                        43824 ... 43866 => {
                            __current_state = 58;
                            continue;
                        }
                        43868 ... 43877 => {
                            __current_state = 58;
                            continue;
                        }
                        43888 ... 44010 => {
                            __current_state = 58;
                            continue;
                        }
                        44012 ... 44013 => {
                            __current_state = 58;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_state = 58;
                            continue;
                        }
                        44032 ... 55203 => {
                            __current_state = 58;
                            continue;
                        }
                        55216 ... 55238 => {
                            __current_state = 58;
                            continue;
                        }
                        55243 ... 55291 => {
                            __current_state = 58;
                            continue;
                        }
                        63744 ... 64109 => {
                            __current_state = 58;
                            continue;
                        }
                        64112 ... 64217 => {
                            __current_state = 58;
                            continue;
                        }
                        64256 ... 64262 => {
                            __current_state = 58;
                            continue;
                        }
                        64275 ... 64279 => {
                            __current_state = 58;
                            continue;
                        }
                        64285 ... 64296 => {
                            __current_state = 58;
                            continue;
                        }
                        64298 ... 64310 => {
                            __current_state = 58;
                            continue;
                        }
                        64312 ... 64316 => {
                            __current_state = 58;
                            continue;
                        }
                        64318 => /* '\u{fb3e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        64320 ... 64321 => {
                            __current_state = 58;
                            continue;
                        }
                        64323 ... 64324 => {
                            __current_state = 58;
                            continue;
                        }
                        64326 ... 64433 => {
                            __current_state = 58;
                            continue;
                        }
                        64467 ... 64829 => {
                            __current_state = 58;
                            continue;
                        }
                        64848 ... 64911 => {
                            __current_state = 58;
                            continue;
                        }
                        64914 ... 64967 => {
                            __current_state = 58;
                            continue;
                        }
                        65008 ... 65019 => {
                            __current_state = 58;
                            continue;
                        }
                        65024 ... 65039 => {
                            __current_state = 58;
                            continue;
                        }
                        65056 ... 65071 => {
                            __current_state = 58;
                            continue;
                        }
                        65075 ... 65076 => {
                            __current_state = 58;
                            continue;
                        }
                        65101 ... 65103 => {
                            __current_state = 58;
                            continue;
                        }
                        65136 ... 65140 => {
                            __current_state = 58;
                            continue;
                        }
                        65142 ... 65276 => {
                            __current_state = 58;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_state = 58;
                            continue;
                        }
                        65313 ... 65338 => {
                            __current_state = 58;
                            continue;
                        }
                        65343 => /* '\u{ff3f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        65345 ... 65370 => {
                            __current_state = 58;
                            continue;
                        }
                        65382 ... 65470 => {
                            __current_state = 58;
                            continue;
                        }
                        65474 ... 65479 => {
                            __current_state = 58;
                            continue;
                        }
                        65482 ... 65487 => {
                            __current_state = 58;
                            continue;
                        }
                        65490 ... 65495 => {
                            __current_state = 58;
                            continue;
                        }
                        65498 ... 65500 => {
                            __current_state = 58;
                            continue;
                        }
                        65536 ... 65547 => {
                            __current_state = 58;
                            continue;
                        }
                        65549 ... 65574 => {
                            __current_state = 58;
                            continue;
                        }
                        65576 ... 65594 => {
                            __current_state = 58;
                            continue;
                        }
                        65596 ... 65597 => {
                            __current_state = 58;
                            continue;
                        }
                        65599 ... 65613 => {
                            __current_state = 58;
                            continue;
                        }
                        65616 ... 65629 => {
                            __current_state = 58;
                            continue;
                        }
                        65664 ... 65786 => {
                            __current_state = 58;
                            continue;
                        }
                        65856 ... 65908 => {
                            __current_state = 58;
                            continue;
                        }
                        66045 => /* '\u{101fd}' */ {
                            __current_state = 58;
                            continue;
                        }
                        66176 ... 66204 => {
                            __current_state = 58;
                            continue;
                        }
                        66208 ... 66256 => {
                            __current_state = 58;
                            continue;
                        }
                        66272 => /* '\u{102e0}' */ {
                            __current_state = 58;
                            continue;
                        }
                        66304 ... 66335 => {
                            __current_state = 58;
                            continue;
                        }
                        66352 ... 66378 => {
                            __current_state = 58;
                            continue;
                        }
                        66384 ... 66426 => {
                            __current_state = 58;
                            continue;
                        }
                        66432 ... 66461 => {
                            __current_state = 58;
                            continue;
                        }
                        66464 ... 66499 => {
                            __current_state = 58;
                            continue;
                        }
                        66504 ... 66511 => {
                            __current_state = 58;
                            continue;
                        }
                        66513 ... 66517 => {
                            __current_state = 58;
                            continue;
                        }
                        66560 ... 66717 => {
                            __current_state = 58;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_state = 58;
                            continue;
                        }
                        66816 ... 66855 => {
                            __current_state = 58;
                            continue;
                        }
                        66864 ... 66915 => {
                            __current_state = 58;
                            continue;
                        }
                        67072 ... 67382 => {
                            __current_state = 58;
                            continue;
                        }
                        67392 ... 67413 => {
                            __current_state = 58;
                            continue;
                        }
                        67424 ... 67431 => {
                            __current_state = 58;
                            continue;
                        }
                        67584 ... 67589 => {
                            __current_state = 58;
                            continue;
                        }
                        67592 => /* '\u{10808}' */ {
                            __current_state = 58;
                            continue;
                        }
                        67594 ... 67637 => {
                            __current_state = 58;
                            continue;
                        }
                        67639 ... 67640 => {
                            __current_state = 58;
                            continue;
                        }
                        67644 => /* '\u{1083c}' */ {
                            __current_state = 58;
                            continue;
                        }
                        67647 ... 67669 => {
                            __current_state = 58;
                            continue;
                        }
                        67680 ... 67702 => {
                            __current_state = 58;
                            continue;
                        }
                        67712 ... 67742 => {
                            __current_state = 58;
                            continue;
                        }
                        67808 ... 67826 => {
                            __current_state = 58;
                            continue;
                        }
                        67828 ... 67829 => {
                            __current_state = 58;
                            continue;
                        }
                        67840 ... 67861 => {
                            __current_state = 58;
                            continue;
                        }
                        67872 ... 67897 => {
                            __current_state = 58;
                            continue;
                        }
                        67968 ... 68023 => {
                            __current_state = 58;
                            continue;
                        }
                        68030 ... 68031 => {
                            __current_state = 58;
                            continue;
                        }
                        68096 ... 68099 => {
                            __current_state = 58;
                            continue;
                        }
                        68101 ... 68102 => {
                            __current_state = 58;
                            continue;
                        }
                        68108 ... 68115 => {
                            __current_state = 58;
                            continue;
                        }
                        68117 ... 68119 => {
                            __current_state = 58;
                            continue;
                        }
                        68121 ... 68147 => {
                            __current_state = 58;
                            continue;
                        }
                        68152 ... 68154 => {
                            __current_state = 58;
                            continue;
                        }
                        68159 => /* '\u{10a3f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        68192 ... 68220 => {
                            __current_state = 58;
                            continue;
                        }
                        68224 ... 68252 => {
                            __current_state = 58;
                            continue;
                        }
                        68288 ... 68295 => {
                            __current_state = 58;
                            continue;
                        }
                        68297 ... 68326 => {
                            __current_state = 58;
                            continue;
                        }
                        68352 ... 68405 => {
                            __current_state = 58;
                            continue;
                        }
                        68416 ... 68437 => {
                            __current_state = 58;
                            continue;
                        }
                        68448 ... 68466 => {
                            __current_state = 58;
                            continue;
                        }
                        68480 ... 68497 => {
                            __current_state = 58;
                            continue;
                        }
                        68608 ... 68680 => {
                            __current_state = 58;
                            continue;
                        }
                        68736 ... 68786 => {
                            __current_state = 58;
                            continue;
                        }
                        68800 ... 68850 => {
                            __current_state = 58;
                            continue;
                        }
                        69632 ... 69702 => {
                            __current_state = 58;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_state = 58;
                            continue;
                        }
                        69759 ... 69818 => {
                            __current_state = 58;
                            continue;
                        }
                        69840 ... 69864 => {
                            __current_state = 58;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_state = 58;
                            continue;
                        }
                        69888 ... 69940 => {
                            __current_state = 58;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_state = 58;
                            continue;
                        }
                        69968 ... 70003 => {
                            __current_state = 58;
                            continue;
                        }
                        70006 => /* '\u{11176}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70016 ... 70084 => {
                            __current_state = 58;
                            continue;
                        }
                        70090 ... 70092 => {
                            __current_state = 58;
                            continue;
                        }
                        70096 ... 70106 => {
                            __current_state = 58;
                            continue;
                        }
                        70108 => /* '\u{111dc}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70144 ... 70161 => {
                            __current_state = 58;
                            continue;
                        }
                        70163 ... 70199 => {
                            __current_state = 58;
                            continue;
                        }
                        70272 ... 70278 => {
                            __current_state = 58;
                            continue;
                        }
                        70280 => /* '\u{11288}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70282 ... 70285 => {
                            __current_state = 58;
                            continue;
                        }
                        70287 ... 70301 => {
                            __current_state = 58;
                            continue;
                        }
                        70303 ... 70312 => {
                            __current_state = 58;
                            continue;
                        }
                        70320 ... 70378 => {
                            __current_state = 58;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_state = 58;
                            continue;
                        }
                        70400 ... 70403 => {
                            __current_state = 58;
                            continue;
                        }
                        70405 ... 70412 => {
                            __current_state = 58;
                            continue;
                        }
                        70415 ... 70416 => {
                            __current_state = 58;
                            continue;
                        }
                        70419 ... 70440 => {
                            __current_state = 58;
                            continue;
                        }
                        70442 ... 70448 => {
                            __current_state = 58;
                            continue;
                        }
                        70450 ... 70451 => {
                            __current_state = 58;
                            continue;
                        }
                        70453 ... 70457 => {
                            __current_state = 58;
                            continue;
                        }
                        70460 ... 70468 => {
                            __current_state = 58;
                            continue;
                        }
                        70471 ... 70472 => {
                            __current_state = 58;
                            continue;
                        }
                        70475 ... 70477 => {
                            __current_state = 58;
                            continue;
                        }
                        70480 => /* '\u{11350}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70487 => /* '\u{11357}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70493 ... 70499 => {
                            __current_state = 58;
                            continue;
                        }
                        70502 ... 70508 => {
                            __current_state = 58;
                            continue;
                        }
                        70512 ... 70516 => {
                            __current_state = 58;
                            continue;
                        }
                        70784 ... 70853 => {
                            __current_state = 58;
                            continue;
                        }
                        70855 => /* '\u{114c7}' */ {
                            __current_state = 58;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_state = 58;
                            continue;
                        }
                        71040 ... 71093 => {
                            __current_state = 58;
                            continue;
                        }
                        71096 ... 71104 => {
                            __current_state = 58;
                            continue;
                        }
                        71128 ... 71133 => {
                            __current_state = 58;
                            continue;
                        }
                        71168 ... 71232 => {
                            __current_state = 58;
                            continue;
                        }
                        71236 => /* '\u{11644}' */ {
                            __current_state = 58;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_state = 58;
                            continue;
                        }
                        71296 ... 71351 => {
                            __current_state = 58;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_state = 58;
                            continue;
                        }
                        71424 ... 71449 => {
                            __current_state = 58;
                            continue;
                        }
                        71453 ... 71467 => {
                            __current_state = 58;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_state = 58;
                            continue;
                        }
                        71840 ... 71913 => {
                            __current_state = 58;
                            continue;
                        }
                        71935 => /* '\u{118ff}' */ {
                            __current_state = 58;
                            continue;
                        }
                        72384 ... 72440 => {
                            __current_state = 58;
                            continue;
                        }
                        73728 ... 74649 => {
                            __current_state = 58;
                            continue;
                        }
                        74752 ... 74862 => {
                            __current_state = 58;
                            continue;
                        }
                        74880 ... 75075 => {
                            __current_state = 58;
                            continue;
                        }
                        77824 ... 78894 => {
                            __current_state = 58;
                            continue;
                        }
                        82944 ... 83526 => {
                            __current_state = 58;
                            continue;
                        }
                        92160 ... 92728 => {
                            __current_state = 58;
                            continue;
                        }
                        92736 ... 92766 => {
                            __current_state = 58;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_state = 58;
                            continue;
                        }
                        92880 ... 92909 => {
                            __current_state = 58;
                            continue;
                        }
                        92912 ... 92916 => {
                            __current_state = 58;
                            continue;
                        }
                        92928 ... 92982 => {
                            __current_state = 58;
                            continue;
                        }
                        92992 ... 92995 => {
                            __current_state = 58;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_state = 58;
                            continue;
                        }
                        93027 ... 93047 => {
                            __current_state = 58;
                            continue;
                        }
                        93053 ... 93071 => {
                            __current_state = 58;
                            continue;
                        }
                        93952 ... 94020 => {
                            __current_state = 58;
                            continue;
                        }
                        94032 ... 94078 => {
                            __current_state = 58;
                            continue;
                        }
                        94095 ... 94111 => {
                            __current_state = 58;
                            continue;
                        }
                        110592 ... 110593 => {
                            __current_state = 58;
                            continue;
                        }
                        113664 ... 113770 => {
                            __current_state = 58;
                            continue;
                        }
                        113776 ... 113788 => {
                            __current_state = 58;
                            continue;
                        }
                        113792 ... 113800 => {
                            __current_state = 58;
                            continue;
                        }
                        113808 ... 113817 => {
                            __current_state = 58;
                            continue;
                        }
                        113821 ... 113822 => {
                            __current_state = 58;
                            continue;
                        }
                        119141 ... 119145 => {
                            __current_state = 58;
                            continue;
                        }
                        119149 ... 119154 => {
                            __current_state = 58;
                            continue;
                        }
                        119163 ... 119170 => {
                            __current_state = 58;
                            continue;
                        }
                        119173 ... 119179 => {
                            __current_state = 58;
                            continue;
                        }
                        119210 ... 119213 => {
                            __current_state = 58;
                            continue;
                        }
                        119362 ... 119364 => {
                            __current_state = 58;
                            continue;
                        }
                        119808 ... 119892 => {
                            __current_state = 58;
                            continue;
                        }
                        119894 ... 119964 => {
                            __current_state = 58;
                            continue;
                        }
                        119966 ... 119967 => {
                            __current_state = 58;
                            continue;
                        }
                        119970 => /* '\u{1d4a2}' */ {
                            __current_state = 58;
                            continue;
                        }
                        119973 ... 119974 => {
                            __current_state = 58;
                            continue;
                        }
                        119977 ... 119980 => {
                            __current_state = 58;
                            continue;
                        }
                        119982 ... 119993 => {
                            __current_state = 58;
                            continue;
                        }
                        119995 => /* '\u{1d4bb}' */ {
                            __current_state = 58;
                            continue;
                        }
                        119997 ... 120003 => {
                            __current_state = 58;
                            continue;
                        }
                        120005 ... 120069 => {
                            __current_state = 58;
                            continue;
                        }
                        120071 ... 120074 => {
                            __current_state = 58;
                            continue;
                        }
                        120077 ... 120084 => {
                            __current_state = 58;
                            continue;
                        }
                        120086 ... 120092 => {
                            __current_state = 58;
                            continue;
                        }
                        120094 ... 120121 => {
                            __current_state = 58;
                            continue;
                        }
                        120123 ... 120126 => {
                            __current_state = 58;
                            continue;
                        }
                        120128 ... 120132 => {
                            __current_state = 58;
                            continue;
                        }
                        120134 => /* '\u{1d546}' */ {
                            __current_state = 58;
                            continue;
                        }
                        120138 ... 120144 => {
                            __current_state = 58;
                            continue;
                        }
                        120146 ... 120485 => {
                            __current_state = 58;
                            continue;
                        }
                        120488 ... 120512 => {
                            __current_state = 58;
                            continue;
                        }
                        120514 ... 120538 => {
                            __current_state = 58;
                            continue;
                        }
                        120540 ... 120570 => {
                            __current_state = 58;
                            continue;
                        }
                        120572 ... 120596 => {
                            __current_state = 58;
                            continue;
                        }
                        120598 ... 120628 => {
                            __current_state = 58;
                            continue;
                        }
                        120630 ... 120654 => {
                            __current_state = 58;
                            continue;
                        }
                        120656 ... 120686 => {
                            __current_state = 58;
                            continue;
                        }
                        120688 ... 120712 => {
                            __current_state = 58;
                            continue;
                        }
                        120714 ... 120744 => {
                            __current_state = 58;
                            continue;
                        }
                        120746 ... 120770 => {
                            __current_state = 58;
                            continue;
                        }
                        120772 ... 120779 => {
                            __current_state = 58;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_state = 58;
                            continue;
                        }
                        121344 ... 121398 => {
                            __current_state = 58;
                            continue;
                        }
                        121403 ... 121452 => {
                            __current_state = 58;
                            continue;
                        }
                        121461 => /* '\u{1da75}' */ {
                            __current_state = 58;
                            continue;
                        }
                        121476 => /* '\u{1da84}' */ {
                            __current_state = 58;
                            continue;
                        }
                        121499 ... 121503 => {
                            __current_state = 58;
                            continue;
                        }
                        121505 ... 121519 => {
                            __current_state = 58;
                            continue;
                        }
                        124928 ... 125124 => {
                            __current_state = 58;
                            continue;
                        }
                        125136 ... 125142 => {
                            __current_state = 58;
                            continue;
                        }
                        126464 ... 126467 => {
                            __current_state = 58;
                            continue;
                        }
                        126469 ... 126495 => {
                            __current_state = 58;
                            continue;
                        }
                        126497 ... 126498 => {
                            __current_state = 58;
                            continue;
                        }
                        126500 => /* '\u{1ee24}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126503 => /* '\u{1ee27}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126505 ... 126514 => {
                            __current_state = 58;
                            continue;
                        }
                        126516 ... 126519 => {
                            __current_state = 58;
                            continue;
                        }
                        126521 => /* '\u{1ee39}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126523 => /* '\u{1ee3b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126530 => /* '\u{1ee42}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126535 => /* '\u{1ee47}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126537 => /* '\u{1ee49}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126539 => /* '\u{1ee4b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126541 ... 126543 => {
                            __current_state = 58;
                            continue;
                        }
                        126545 ... 126546 => {
                            __current_state = 58;
                            continue;
                        }
                        126548 => /* '\u{1ee54}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126551 => /* '\u{1ee57}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126553 => /* '\u{1ee59}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126555 => /* '\u{1ee5b}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126557 => /* '\u{1ee5d}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126559 => /* '\u{1ee5f}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126561 ... 126562 => {
                            __current_state = 58;
                            continue;
                        }
                        126564 => /* '\u{1ee64}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126567 ... 126570 => {
                            __current_state = 58;
                            continue;
                        }
                        126572 ... 126578 => {
                            __current_state = 58;
                            continue;
                        }
                        126580 ... 126583 => {
                            __current_state = 58;
                            continue;
                        }
                        126585 ... 126588 => {
                            __current_state = 58;
                            continue;
                        }
                        126590 => /* '\u{1ee7e}' */ {
                            __current_state = 58;
                            continue;
                        }
                        126592 ... 126601 => {
                            __current_state = 58;
                            continue;
                        }
                        126603 ... 126619 => {
                            __current_state = 58;
                            continue;
                        }
                        126625 ... 126627 => {
                            __current_state = 58;
                            continue;
                        }
                        126629 ... 126633 => {
                            __current_state = 58;
                            continue;
                        }
                        126635 ... 126651 => {
                            __current_state = 58;
                            continue;
                        }
                        127280 ... 127305 => {
                            __current_state = 58;
                            continue;
                        }
                        127312 ... 127337 => {
                            __current_state = 58;
                            continue;
                        }
                        127344 ... 127369 => {
                            __current_state = 58;
                            continue;
                        }
                        131072 ... 173782 => {
                            __current_state = 58;
                            continue;
                        }
                        173824 ... 177972 => {
                            __current_state = 58;
                            continue;
                        }
                        177984 ... 178205 => {
                            __current_state = 58;
                            continue;
                        }
                        178208 ... 183969 => {
                            __current_state = 58;
                            continue;
                        }
                        194560 ... 195101 => {
                            __current_state = 58;
                            continue;
                        }
                        917760 ... 917999 => {
                            __current_state = 58;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 92;
                            continue;
                        }
                        _ => {
                            __current_state = 60;
                            continue;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 92;
                            continue;
                        }
                        _ => {
                            __current_state = 60;
                            continue;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 93;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 93;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_state = 94;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 94;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        85 => /* 'U' */ {
                            __current_state = 95;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 95;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 96;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 96;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 97;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 97;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 98;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 98;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 99;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 99;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                72 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_state = 100;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 100;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                73 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                74 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                75 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                76 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                77 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                78 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                79 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                80 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                81 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                82 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                83 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        72 => /* 'H' */ {
                            __current_state = 101;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 101;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                84 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                85 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                86 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                87 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((43, __index + 1));
                            __current_state = 102;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((43, __index + 1));
                            __current_state = 102;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                88 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                89 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                90 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                91 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 103;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                92 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 92;
                            continue;
                        }
                        _ => {
                            __current_state = 60;
                            continue;
                        }
                    }
                }
                93 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        85 => /* 'U' */ {
                            __current_state = 104;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 104;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                94 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 105;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 105;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                95 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 106;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 106;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                96 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_state = 107;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 107;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                97 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        75 => /* 'K' */ {
                            __current_state = 108;
                            continue;
                        }
                        107 => /* 'k' */ {
                            __current_state = 108;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                98 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 109;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 109;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                99 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 110;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 110;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                100 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_match = Some((37, __index + 1));
                            __current_state = 111;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((37, __index + 1));
                            __current_state = 111;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                101 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        74 => /* 'J' */ {
                            __current_state = 112;
                            continue;
                        }
                        106 => /* 'j' */ {
                            __current_state = 112;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                102 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                103 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 113;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 114;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                104 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 115;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 115;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                105 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 116;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 116;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                106 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 117;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 117;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                107 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        72 => /* 'H' */ {
                            __current_state = 118;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 118;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                108 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 119;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 119;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                109 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 120;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 120;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                110 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        68 => /* 'D' */ {
                            __current_match = Some((35, __index + 1));
                            __current_state = 121;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((35, __index + 1));
                            __current_state = 121;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                111 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        95 => /* '_' */ {
                            __current_state = 122;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                112 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 123;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 123;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                113 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        47 => /* '/' */ {
                            __current_state = 124;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                114 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_state = 113;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                115 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 125;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 125;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                116 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                117 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 126;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 126;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                118 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 127;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 127;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                119 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 128;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                120 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 129;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 129;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                121 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                122 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        68 => /* 'D' */ {
                            __current_state = 130;
                            continue;
                        }
                        83 => /* 'S' */ {
                            __current_state = 131;
                            continue;
                        }
                        84 => /* 'T' */ {
                            __current_state = 132;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_state = 130;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 131;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 132;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                123 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_state = 133;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 133;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                124 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        47 => /* '/' */ {
                            __current_state = 134;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                125 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_state = 135;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 135;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                126 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                127 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 136;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 136;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                128 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                129 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                130 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_state = 137;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 137;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                131 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 138;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 138;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                132 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        79 => /* 'O' */ {
                            __current_match = Some((40, __index + 1));
                            __current_state = 139;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((40, __index + 1));
                            __current_state = 139;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                133 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        95 => /* '_' */ {
                            __current_state = 140;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                134 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 8 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        14 ... 31 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        33 => /* '!' */ {
                            __current_match = Some((45, __index + 1));
                            __current_state = 141;
                            continue;
                        }
                        35 ... 132 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        134 ... 159 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        161 ... 5759 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        5761 ... 8191 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8203 ... 8231 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8234 ... 8238 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8240 ... 8286 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8288 ... 12287 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        12289 ... 1114111 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                135 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        89 => /* 'Y' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 142;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 142;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                136 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                137 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        77 => /* 'M' */ {
                            __current_state = 143;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 143;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                138 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        67 => /* 'C' */ {
                            __current_state = 144;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_state = 144;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                139 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                140 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        83 => /* 'S' */ {
                            __current_state = 145;
                            continue;
                        }
                        85 => /* 'U' */ {
                            __current_state = 146;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 145;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 146;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                141 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 8 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        14 ... 31 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        33 => /* '!' */ {
                            __current_match = Some((45, __index + 1));
                            __current_state = 141;
                            continue;
                        }
                        35 ... 132 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        134 ... 159 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        161 ... 5759 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        5761 ... 8191 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8203 ... 8231 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8234 ... 8238 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8240 ... 8286 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        8288 ... 12287 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        12289 ... 1114111 => {
                            __current_match = Some((45, __index + __ch.len_utf8()));
                            __current_state = 141;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                142 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                143 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 => /* 'A' */ {
                            __current_state = 147;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 147;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                144 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 148;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 148;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                145 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 149;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 149;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                146 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 150;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 150;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                147 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_state = 151;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 151;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                148 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 152;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 152;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                149 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        67 => /* 'C' */ {
                            __current_state = 153;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_state = 153;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                150 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        76 => /* 'L' */ {
                            __current_match = Some((42, __index + 1));
                            __current_state = 154;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((42, __index + 1));
                            __current_state = 154;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                151 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_match = Some((38, __index + 1));
                            __current_state = 155;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((38, __index + 1));
                            __current_state = 155;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                152 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((39, __index + 1));
                            __current_state = 156;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((39, __index + 1));
                            __current_state = 156;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                153 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 157;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 157;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                154 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                155 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                156 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                157 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        69 => /* 'E' */ {
                            __current_state = 158;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_state = 158;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                158 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((41, __index + 1));
                            __current_state = 159;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((41, __index + 1));
                            __current_state = 159;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                159 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, job::Line, usize),
) -> job::Line
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, job::Var, usize),
) -> job::Var
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, job::Command, usize),
) -> job::Command
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, cmd, _): (usize, job::Command, usize),
) -> job::Line
{
    job::Line::Cmd(cmd)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, job::Var, usize),
) -> job::Line
{
    job::Line::VarSet(v)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Line
{
    job::Line::Comment
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::EmailDomain(v.to_string())
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::EmailSecret(v.to_string())
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::EmailRecip(v.to_string())
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::PjSecret(v.to_string())
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::PjUrl(v.to_string())
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> job::Var
{
    job::Var::DataDir( v.to_string())
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, job::Time, usize),
    (_, a, _): (usize, job::Action, usize),
) -> job::Command
{
    job::Command { time:t, act:a }
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, u, _): (usize, &'input str, usize),
) -> job::Action
{
    job::Action { url:u.to_owned(), contact:None }
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, u, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, job::Contact, usize),
) -> job::Action
{
    job::Action { url:u.to_owned(), contact:Some(c) }
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Contact
{
    job::Contact::Email
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Contact
{
    job::Contact::Text
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Contact
{
    job::Contact::LogAll
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, Vec<job::Value>, usize),
    (_, b, _): (usize, Vec<job::Value>, usize),
    (_, c, _): (usize, Vec<job::Value>, usize),
    (_, d, _): (usize, Vec<job::Value>, usize),
    (_, e, _): (usize, Vec<job::Value>, usize),
) -> job::Time
{
    job::Time { minute:a, hour:b, date:c, month:d, weekday:e }
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, job::Time, usize),
) -> job::Time
{
    (__0)
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, ZERO, ONE , ONE , STAR)
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, ZERO, ONE , ONE , STAR)
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, ZERO, ONE , STAR, STAR)
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, ZERO, STAR, STAR, ZERO)
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, ZERO, STAR, STAR, STAR)
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::Time
{
    job::Time::from(ZERO, STAR, STAR, STAR, STAR)
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<job::Value>, usize),
) -> Vec<job::Value>
{
    (__0)
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, d, _): (usize, u8, usize),
) -> Vec<job::Value>
{
    vec![job::Value::Constant(d)]
}

pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<job::Value>, usize),
) -> Vec<job::Value>
{
    (__0)
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, m, _): (usize, u8, usize),
) -> Vec<job::Value>
{
    vec![job::Value::Constant(m)]
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<job::Value>, usize),
) -> Vec<job::Value>
{
    (__0)
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u8, usize),
) -> job::Value
{
    job::Value::Constant(n)
}

pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, cv, _): (usize, job::ContVal, usize),
) -> job::Value
{
    job::Value::CV(cv)
}

pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, cv, _): (usize, job::ContVal, usize),
    (_, s, _): (usize, u8, usize),
) -> job::Value
{
    job::Value::Skip(cv, s)
}

pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, u8, usize),
) -> u8
{
    (__0)
}

pub fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> job::ContVal
{
    job::ContVal::Asterisk
}

pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u8, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, u8, usize),
) -> job::ContVal
{
    job::ContVal::Range(n,m+1)
}

pub fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    0
}

pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    1
}

pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    2
}

pub fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    3
}

pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    4
}

pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    5
}

pub fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    6
}

pub fn __action47<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    1
}

pub fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    2
}

pub fn __action49<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    3
}

pub fn __action50<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    4
}

pub fn __action51<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    5
}

pub fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    6
}

pub fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    7
}

pub fn __action54<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    8
}

pub fn __action55<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    9
}

pub fn __action56<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    10
}

pub fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    11
}

pub fn __action58<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    12
}

pub fn __action59<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> u8
{
    u8::from_str(n).unwrap()
}

pub fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

pub fn __action61<
    'input,
>(
    input: &'input str,
    (_, val, _): (usize, job::Value, usize),
    (_, vec, _): (usize, ::std::vec::Vec<job::Value>, usize),
) -> Vec<job::Value>
{
    {
        let mut vec = vec;
        vec.push(val);
        vec
    }
}

pub fn __action62<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<job::Value>
{
    vec![]
}

pub fn __action63<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<job::Value>, usize),
) -> ::std::vec::Vec<job::Value>
{
    v
}

pub fn __action64<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, job::Value, usize),
) -> job::Value
{
    (__0)
}

pub fn __action65<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, job::Value, usize),
) -> ::std::vec::Vec<job::Value>
{
    vec![__0]
}

pub fn __action66<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<job::Value>, usize),
    (_, e, _): (usize, job::Value, usize),
) -> ::std::vec::Vec<job::Value>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, job::Value, usize),
) -> ::std::vec::Vec<job::Value>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action64(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __temp0,
    )
}

pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<job::Value>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, job::Value, usize),
) -> ::std::vec::Vec<job::Value>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action64(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __0,
        __temp0,
    )
}

pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, job::Value, usize),
) -> Vec<job::Value>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action62(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __temp0,
    )
}

pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, job::Value, usize),
    __1: (usize, ::std::vec::Vec<job::Value>, usize),
) -> Vec<job::Value>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
