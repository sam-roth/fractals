use std::collections::HashMap;
use lsystem::{LSystem, Sym};


pub fn lsystem_from_strs(fwds: &str,
                         vars: &str,
                         axiom: &str,
                         prods: &[(char, &str)]) -> LSystem {
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

    LSystem::new(axiom_instrs, prods_map)
}
