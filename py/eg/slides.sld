uxf 1 Slides 0.1.0
#<This is a simple example of an application-specific use of UXF.
See slides[12].py for examples of converting this format to HTML.>
= Slide title body
= #<title> h1 content
= #<subtitle> h2 content
= #<bullet item> B content
= #<para> p content
= #<image> img content image:bytes
= #<monospace inline> m content
= #<monospace block> pre content
= #<italic> i content
= url content link
= #<newline with no content> nl
(Slide

(h1 <The Uniform eXchange Format>)
[
  (h2 <A Brief Introduction>)
  <Uniform eXchange Format> (i <UXF>) <is a plain text human readable
  optionally typed storage format.> (i <UXF>) <may serve as a convenient
  alternative to>
  (m <.csv, ini, json, sqlite, toml, xml,>) <or> (m <yaml.>)
  (p
    (img (p (i <UXF>) <Logo>)
      (:89504e470d0a1a0a0000000d49484452000000180000001d0806000000b0baac4b0000000473424954080808087c08648800000009704859730000012600000126015ffb12280000001974455874536f667477617265007777772e696e6b73636170652e6f72679bee3c1a0000049a4944415448899d967b6c536518c69fefdc7adbba8db5abeb2e1d731bbbe2580f640c655e32071b97b848a32420a22078fb8799282290210b73930413251a9dad0846d08981190d89036246cc1c2872999932a2026b601bf4c64edb733eff28ad5d77a3bcff7dcffb9ce7f79e2fe7e4fb484ba99de24ef15ad6b9f083a2874551ec434cb5963b3a9520ad8bd5a7aa3431a9838916f844d604e0c189ccacc0a8e20907007daea681bb5bb3da201c3715681f9bcae3bb2661f8bc07a0004848bb6b40cd9e8a56b7db7d642acf2f6f0f2c05b0231c1e17203f3f5f02f0db549e9612477974785c0000d859b42f9de5947993f5a942f362f2e303708cb200941e9aacaf36089ed121ff188d99c47b4fc5aa98847143c513200bf28939eb73ba1915b97fa2fe489f4fedbd3a9a74cf80cd67d65ec7462c98acdf52e2780604f6686dca2d6a2e74a4ee2ef978c664fddd0f3832a61b6a5240f39c7623cbd32e26557361ff739d9671d3967eba214871c9bef69b4d71032828e1824c2728caa4e18069e8f2adf3ed1bbfcefd3fdcbe09207ba94285eb675c6d5f6dfd6e5d5c000242b36a8d9fb3aad0577ddbe9d7792e4ae73e5add51b0abd4de08a02dec355af5b0d4a72d8f0b00004fb7d5bf975d6bdc1281dcf06b3c7f78cf11a035ec312f4c41f1ba8c1f090b5bdc0000b035d7edb4d41a1b594dc816f0ca7c24bc3a05856bccc782727089288abe7b0200c08ae6ba77557afe58b4c6a9195816194e78bc9e65555555b7a77a7edaffa0a5c4d1e4734a35d15a7054416fcbe5f986627d3180d3619d003c8d361280210c02e175c027038010092f75bc0342df0aafd3ab92c1eb580080ff664018bae83ab5ffd5a3d6709f32481dfba66c90213ce389003c32fc9ea029146e6f03e86be19ea5ce88e2f59947336b535ee4751c05006924205c3f73eb94e3a58ef92102cd8b06087ad6cbf01af6af884281e173de0500905ca093084322e17936d321000db6a6257bb317cf789e4f6029001096f039b5a6f70180e1c9e3d180448ba68f610819734add38edaaeaeaeae25ee858f1a6b93ab963e6f234e4d94c07dd6ef74a51140300f0e4f6baf6ac9ad435fa5c8d6c7d7da64b9b2e6cd855e658ac04687654383446e104d959b42f5d5029ff2a41cadc9908792b4cdb1ab62c6a02809e9e9e650303039d369b4d464cf5f6f656524afddd4ff5fd2969e51e45a105e15ee11a33b21e35cc2500f0e11387bebdd9ef5d16d9bb244e49ced7d63f6b6ff83e3634b65a677f96a628f241805487355d861ae2b6dce39555f31e0901961ed1fa6f7b9cbe41297260301ca11a037fc0ed1cddfac6ef6b2fc506b7ccfac40c815b45081aa9420d619d5533b06e9e2925e5e82a2a2a2a2e448ed083db3bcb9dddae9f7d572521368c55312384257f530523344835003294a09219ebe3752cca5eb128338a742bad56eb9700c6de017e38d055e83ce93a39d83d62a4ca749b33b652cb12306bb5d9a3310aab44513c1cd6632f01e8efef575dfae99f1d4367bd2f0f9d756b7c4e7fac2552829e43eaec44981f4a0924cfd27ec171dce6f2f2f22bd19e71806890cbe5aaf70d4acbbdd7a4b9f2a86c0e8e2a3a5ec732ac8af5aa8dfc9584fbd4bf2a508ecbb27cb8b2b2d23951ce7f1e84995b8ac93cb90000000049454e44ae426082:)
    )
  )
]

(h1 <Overview>)
[
  (B <Motivation>)
  (B <Main Characteristics>)
  (B <Overall Structure>)
  (B <Collection Types>)
  (B <Scalar Types>)
  (B <Conclusion>)
  (B (i <UXF>) <source code for these slides and> (m <.py>) <source code for
  the program than converts them to> (m <.html>)<.>)
]

(h1 <Motivation>)
[
  (B [(m <.csv>) <has only one table, &amp; isn't standardized or typed>])
  (B [(m <.ini>) <only nests 2-3 deep, &amp; isn't standardized or typed>])
  (B [(m <.json>) <has lots of punctuation, no date/time support and is verbose>])
  (B [(m <.sqlite>) <is binary so can't be hand edited>])
  (B [(m <.xml>) <is heavyweight and isn't nice to hand edit>])
  (B [(i <UXF>) <can hold any number of tables (and maps, and lists, all
  nestable)>])
  (B [(i <UXF>) <is optionally typed>])
  (B [(i <UXF>) <supports date/times and binary data>])
  (B [(i <UXF>) <is fairly lightweight and hand editable>])
]

(h1 <Overall Structure>)
[
  (p <A> (i <UXF>) <file consists of a header line, then optional table type
  definitions, then a single list, map, or table containing the data. Since
  lists, maps, and tables can nest, this allows for any arbitrary data
  structure of any amount of data.>)
  (m <uxf 1> (nl))
  (m <= #&lt;optional ttypes go here&gt; Point x y> (nl))
  (m <[ #&lt;data goes here&gt;> (nl))
  (m <(Point 5 9)> (nl))
  (m <&lt;Some text&gt;> (nl))
  (m <2022-04-29> (nl))
  (m <9.8e6> (nl))
  (m <]> (nl))
  (nl)
]

(h1 <Scalar Types>)
[
  (B (i <UXF>) <supports 8 scalar types:>
    (B (m <bool>) (m <int>) (m <real>))
    (B (m <bytes>) (m <str>))
    (B (m <date>) (m <datetime>))
    (B (m <null>))
  )
]

(h1 <Collection Types>)
[
  (B (i <UXF>) <supports 3 collection types:>
    (B (m <list>))
    (B (m <map>))
    (B (m <table>))
  )
]

(h1 <Conclusion>)
[
  (p <These slides are purely to show an example of how flexible the>
  (i <UXF>) <format is.>)
  (p <For more about UXF, visit the> (url <UXF home page>
  <https://github.com/mark-summerfield/uxf>) <.>)
]

)
