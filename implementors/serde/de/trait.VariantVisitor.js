(function() {var implementors = {};
implementors['serde'] = [];implementors['bincode'] = ["impl&lt;'a, R: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a>&gt; <a class='trait' href='serde/de/trait.VariantVisitor.html' title='serde::de::VariantVisitor'>VariantVisitor</a> for <a class='struct' href='bincode/serde/struct.Deserializer.html' title='bincode::serde::Deserializer'>Deserializer</a>&lt;'a, R&gt;",];implementors['wire'] = ["impl&lt;'a, R&gt; <a class='trait' href='serde/de/trait.VariantVisitor.html' title='serde::de::VariantVisitor'>VariantVisitor</a> for <a class='struct' href='bincode/serde/reader/struct.Deserializer.html' title='bincode::serde::reader::Deserializer'>Deserializer</a>&lt;'a, R&gt; <span class='where'>where R: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a></span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
