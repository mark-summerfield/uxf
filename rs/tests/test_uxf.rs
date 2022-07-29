// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uxf::event::Event;
    use uxf::Uxf;

    // TODO
    /*
    #[test]
    fn t_uxf_on_error() {
        let mut events = Vec::<Event>::new();
        let mut uxo = Uxf::new(
            "custom 1",
            "comment 1",
            Some(Box::new(|event| {
                events.push(event.clone());
                Ok(())
            })),
        );
        ...
        assert!(&events.is_empty());
    }
    */

    #[test]
    fn t_uxf_strings() {
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        uxo.set_custom("Geo 1.0.0");
        uxo.set_comment("A Geographical format");
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 Geo 1.0.0\n#<A Geographical format>\n[]\n"
        );
        let mut uxo = Uxf::new("custom 1", "comment 1", None);
        assert_eq!(
            format!("{:?}", uxo),
            "Uxf { custom: \"custom 1\", comment: \"comment 1\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
        );
        assert_eq!(uxo.to_string(), "uxf 1.0 custom 1\n#<comment 1>\n[]\n");
        uxo.set_comment("New text");
        assert_eq!(
            format!("{:?}", uxo),
            "Uxf { custom: \"custom 1\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
        );
        assert_eq!(uxo.to_string(), "uxf 1.0 custom 1\n#<New text>\n[]\n");
        uxo.set_custom("Dummy format");
        assert_eq!(format!("{:?}", uxo),
        "Uxf { custom: \"Dummy format\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }");
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 Dummy format\n#<New text>\n[]\n"
        );
        // TODO
    }
}
