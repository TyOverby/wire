(function() {var implementors = {};
implementors['rustc_serialize'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='enum' href='rustc_serialize/json/enum.Json.html' title='rustc_serialize::json::Json'>Json</a>",];implementors['serde'] = ["impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='serde/bytes/struct.Bytes.html' title='serde::bytes::Bytes'>Bytes</a>&lt;'a&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='serde/bytes/struct.ByteBuf.html' title='serde::bytes::ByteBuf'>ByteBuf</a>",];implementors['libc'] = [];implementors['num'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='num/bigint/struct.BigUint.html' title='num::bigint::BigUint'>BigUint</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='enum' href='num/bigint/enum.Sign.html' title='num::bigint::Sign'>Sign</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='num/bigint/struct.BigInt.html' title='num::bigint::BigInt'>BigInt</a>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='num/rational/struct.Ratio.html' title='num::rational::Ratio'>Ratio</a>&lt;T&gt; <span class='where'>where T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html' title='core::ops::Mul'>Mul</a>&lt;T, Output=T&gt; + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a></span>",];implementors['bincode'] = ["impl&lt;'a, T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> + 'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='bincode/struct.RefBox.html' title='bincode::RefBox'>RefBox</a>&lt;'a, T&gt;","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='bincode/struct.StrBox.html' title='bincode::StrBox'>StrBox</a>&lt;'a&gt;","impl&lt;'a, T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> + 'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='bincode/struct.SliceBox.html' title='bincode::SliceBox'>SliceBox</a>&lt;'a, T&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='enum' href='bincode/enum.SizeLimit.html' title='bincode::SizeLimit'>SizeLimit</a>",];implementors['unreliable_message'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='unreliable_message/msgqueue/struct.MsgId.html' title='unreliable_message::msgqueue::MsgId'>MsgId</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='unreliable_message/msgqueue/struct.PieceNum.html' title='unreliable_message::msgqueue::PieceNum'>PieceNum</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='unreliable_message/msgqueue/struct.MsgChunk.html' title='unreliable_message::msgqueue::MsgChunk'>MsgChunk</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a> for <a class='struct' href='unreliable_message/msgqueue/struct.CompleteMessage.html' title='unreliable_message::msgqueue::CompleteMessage'>CompleteMessage</a>",];implementors['wire'] = ["impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;<a class='struct' href='bincode/refbox/struct.RefBox.html' title='bincode::refbox::RefBox'>RefBox</a>&lt;'a, T&gt;&gt; for <a class='struct' href='bincode/refbox/struct.RefBox.html' title='bincode::refbox::RefBox'>RefBox</a>&lt;'a, T&gt; <span class='where'>where T: 'a + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;T&gt;</span>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;<a class='struct' href='bincode/refbox/struct.StrBox.html' title='bincode::refbox::StrBox'>StrBox</a>&lt;'a&gt;&gt; for <a class='struct' href='bincode/refbox/struct.StrBox.html' title='bincode::refbox::StrBox'>StrBox</a>&lt;'a&gt;","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;<a class='struct' href='bincode/refbox/struct.SliceBox.html' title='bincode::refbox::SliceBox'>SliceBox</a>&lt;'a, T&gt;&gt; for <a class='struct' href='bincode/refbox/struct.SliceBox.html' title='bincode::refbox::SliceBox'>SliceBox</a>&lt;'a, T&gt; <span class='where'>where T: 'a + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;T&gt;</span>","impl&lt;'a, A, B&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;<a class='enum' href='bincode/refbox/enum.RefBoxInner.html' title='bincode::refbox::RefBoxInner'>RefBoxInner</a>&lt;'a, A, B&gt;&gt; for <a class='enum' href='bincode/refbox/enum.RefBoxInner.html' title='bincode::refbox::RefBoxInner'>RefBoxInner</a>&lt;'a, A, B&gt; <span class='where'>where A: 'a + <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;A&gt; + ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>, B: <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;B&gt;</span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html' title='core::cmp::PartialOrd'>PartialOrd</a>&lt;<a class='enum' href='wire/tcp/enum.SizeLimit.html' title='wire::tcp::SizeLimit'>SizeLimit</a>&gt; for <a class='enum' href='wire/tcp/enum.SizeLimit.html' title='wire::tcp::SizeLimit'>SizeLimit</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
