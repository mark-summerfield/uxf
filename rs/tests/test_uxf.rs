// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uxf::event::Event;
    use uxf::Uxf;

    #[test]
    fn t_uxf_strings() {
        /*
        let mut events = Vec::<Event>::new();
        let mut uxo = Uxf::new(
            "custom 1",
            "comment 1",
            Some(Box::new(|event| {
                events.push(event.clone());
                Ok(())
            })),
        );
        */
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
        /*
        assert!(&events.is_empty());
        */
        // TODO
    }
}
