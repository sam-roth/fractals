use std::collections::HashMap;
use lsystem::{LSystem, Sym};
use rustc_serialize::json;


pub fn lsystem_from_strs(fwds: &str,
                         vars: &str,
                         axiom: &str,
                         prods: &[(char, &str)],
                         angle_radians: f64) -> LSystem {
    let mut map = HashMap::new();

    let consts = [('+', Sym::Plus),
                  ('-', Sym::Minus),
                  ('[', Sym::Push),
                  (']', Sym::Pop)];

    map.extend(consts.iter().cloned());
    map.extend(fwds.chars().enumerate().map(|(i, el)| (el, Sym::Fwd(i))));
    map.extend(vars.chars().enumerate().map(|(i, el)| (el, Sym::Var(i))));

    let axiom_instrs: Vec<_> = axiom
        .chars()
        .map(|ch| *map.get(&ch).expect("Unknown key"))
        .collect();

    let prods_map: HashMap<_, _> = prods.iter()
        .map(|&(ch, expr)| {
            let key = *map
                .get(&ch)
                .expect("Unknown key");
            let instrs: Vec<_> = expr
                .chars()
                .map(|ch| *map.get(&ch).expect("Unknown key"))
                .collect();
            (key, instrs)
        })
        .collect();

    LSystem::new(axiom_instrs, prods_map, angle_radians)
}

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct LSystemStrs {
    fwds: String,
    vars: String,
    axiom: String,
    prods: HashMap<char, String>,
    angle: f64,
}

pub fn parse_lsystem(source: &str) -> json::DecodeResult<LSystem> {
    let strs: LSystemStrs = try!(json::decode(source));
    let prods: Vec<(char, &str)> = strs.prods.iter().map(|(c, s)| (*c, &s[..])).collect();

    Ok(lsystem_from_strs(&strs.fwds, &strs.vars, &strs.axiom, &prods, strs.angle.to_radians()))
}
