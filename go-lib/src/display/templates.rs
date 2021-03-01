use board::stones::stone::Stone;
use rust_tools::screen::layout::layout::L;
use rust_tools::screen::layout::template::Template;

pub struct GoTemplate {}

impl GoTemplate {
    pub fn color_stats(stone:Stone) -> Template {
        let stones = L::ptr("");
        let groups = L::ptr("");
        let captured = L::ptr("");
        let mut res = Template::new(L::hori(vec![
            L::str(&format!("{} : ", stone)),
            L::str2(&stones),
            L::str(" stones, "),
            L::str2(&groups),
            L::str(" stones, "),
            L::str2(&captured),
            L::str(" captured"),
        ]));
        res.register(0, &stones);
        res.register(1, &groups);
        res.register(2, &captured);
        res
    }

    // pub fn stats_template(stats: &BoardStats) -> Template {
    //     let black = &stats.black.template;
    //     let white = &stats.white.template;
    //     let none = &stats.none.template;
    //
    //     let mut res = Template::new(L::vert(vec![
    //         black.template.clone(),
    //         white.template.clone(),
    //         none.template.clone(),
    //     ]));
    //     let mut i = 0;
    //     for (&k, v) in &black.vars {
    //         res.register(k, &v);
    //     }
    //     for (&k, v) in &white.vars {
    //         res.register(k, &v);
    //     }
    //     for (&k, v) in &none.vars {
    //         res.register(k, &v);
    //     }
    //
    //     res
    // }
}